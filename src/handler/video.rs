use super::camera;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::web::Bytes;
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};

impl ResponseError for camera::Error {}

pub async fn forward_video(
    id: Identity,
    _req: HttpRequest,
    _body: web::Bytes,
) -> Result<HttpResponse, Error> {
    // Short return
    if !id.identity().is_some() {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let cam = camera::create(0).unwrap();
    let mut cam = cam.fps(30.0)?.resolution(320, 180)?.start().unwrap();

    // let mut client_resp = HttpResponse::build(StatusCode::OK);

    let pic = cam.next().unwrap();
    // let stream = stream::iter(cam.next());
    let response = HttpResponse::Ok()
        .set(ContentType::jpeg())
        .body(Bytes::from(pic.to_vec()));

    Ok(response)
}
