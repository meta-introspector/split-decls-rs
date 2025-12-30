// Generated macro for fn_factory (function)
macro_rules! Depcrate_fn_servicefn_factory {
() => {
// Module: crate::fn_service
// Provides: {"fn_factory"}
// Dependencies: {}
# [doc = " Create `ServiceFactory` for function that can produce services"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use std::io;"] # [doc = " use actix_service::{fn_factory, fn_service, Service, ServiceFactory};"] # [doc = " use futures_util::future::ok;"] # [doc = ""] # [doc = " /// Service that divides two usize values."] # [doc = " async fn div((x, y): (usize, usize)) -> Result<usize, io::Error> {"] # [doc = "     if y == 0 {"] # [doc = "         Err(io::Error::new(io::ErrorKind::Other, \"divide by zero\"))"] # [doc = "     } else {"] # [doc = "         Ok(x / y)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " #[actix_rt::main]"] # [doc = " async fn main() -> io::Result<()> {"] # [doc = "     // Create service factory that produces `div` services"] # [doc = "     let factory = fn_factory(|| {"] # [doc = "         ok::<_, io::Error>(fn_service(div))"] # [doc = "     });"] # [doc = ""] # [doc = "     // construct new service"] # [doc = "     let srv = factory.new_service(()).await?;"] # [doc = ""] # [doc = "     // now we can use `div` service"] # [doc = "     let result = srv.call((10, 20)).await?;"] # [doc = ""] # [doc = "     println!(\"10 / 20 = {}\", result);"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub fn fn_factory < F , Cfg , Srv , Req , Fut , Err > (f : F) -> FnServiceNoConfig < F , Cfg , Srv , Req , Fut , Err > where F : Fn () -> Fut , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { FnServiceNoConfig :: new (f) }
};
}
