// Generated macro for fn_factory_with_config (function)
macro_rules! Depcrate_fn_servicefn_factory_with_config {
() => {
// Module: crate::fn_service
// Provides: {"fn_factory_with_config"}
// Dependencies: {}
# [doc = " Create `ServiceFactory` for function that accepts config argument and can produce services"] # [doc = ""] # [doc = " Any function that has following form `Fn(Config) -> Future<Output = Service>` could act as"] # [doc = " a `ServiceFactory`."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use std::io;"] # [doc = " use actix_service::{fn_factory_with_config, fn_service, Service, ServiceFactory};"] # [doc = " use futures_util::future::ok;"] # [doc = ""] # [doc = " #[actix_rt::main]"] # [doc = " async fn main() -> io::Result<()> {"] # [doc = "     // Create service factory. factory uses config argument for"] # [doc = "     // services it generates."] # [doc = "     let factory = fn_factory_with_config(|y: usize| {"] # [doc = "         ok::<_, io::Error>(fn_service(move |x: usize| ok::<_, io::Error>(x * y)))"] # [doc = "     });"] # [doc = ""] # [doc = "     // construct new service with config argument"] # [doc = "     let srv = factory.new_service(10).await?;"] # [doc = ""] # [doc = "     let result = srv.call(10).await?;"] # [doc = "     assert_eq!(result, 100);"] # [doc = ""] # [doc = "     println!(\"10 * 10 = {}\", result);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub fn fn_factory_with_config < F , Fut , Cfg , Srv , Req , Err > (f : F ,) -> FnServiceConfig < F , Fut , Cfg , Srv , Req , Err > where F : Fn (Cfg) -> Fut , Fut : Future < Output = Result < Srv , Err > > , Srv : Service < Req > , { FnServiceConfig :: new (f) }
};
}
