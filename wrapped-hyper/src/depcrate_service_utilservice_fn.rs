// Generated macro for service_fn (function)
macro_rules! Depcrate_service_utilservice_fn {
() => {
// Module: crate::service::util
// Provides: {"service_fn"}
// Dependencies: {}
# [doc = " Create a `Service` from a function."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::Bytes;"] # [doc = " use hyper::{body, Request, Response, Version};"] # [doc = " use http_body_util::Full;"] # [doc = " use hyper::service::service_fn;"] # [doc = ""] # [doc = " let service = service_fn(|req: Request<body::Incoming>| async move {"] # [doc = "     if req.version() == Version::HTTP_11 {"] # [doc = "         Ok(Response::new(Full::<Bytes>::from(\"Hello World\")))"] # [doc = "     } else {"] # [doc = "         // Note: it's usually better to return a Response"] # [doc = "         // with an appropriate StatusCode instead of an Err."] # [doc = "         Err(\"not HTTP/1.1, abort connection\")"] # [doc = "     }"] # [doc = " });"] # [doc = " ```"] pub fn service_fn < F , R , S > (f : F) -> ServiceFn < F , R > where F : Fn (Request < R >) -> S , S : Future , { ServiceFn { f , _req : PhantomData , } }
};
}
