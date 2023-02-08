use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct Asset {
    symbol: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Market {
    base_asset: Asset,
    quote_asset: Asset,
    orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    price: f64,
    asset: Asset,
    side: OrderSide,
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum OrderSide {
    Buy,
    Sell,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(coin)
            .service(asset)
            .app_data(web::JsonConfig::default().limit(4096))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn coin() -> impl Responder {
    let bitcoin = Asset {
        name: "Bitcoin".to_string(),
        symbol: "BTC".to_string(),
    };
    HttpResponse::Ok().json(bitcoin)
}

#[post("/asset/new")]
async fn asset(asset: web::Json<Asset>) -> impl Responder {
    println!("Asset: {:?}", &asset);
    return HttpResponse::Ok().json(asset.0);
}
