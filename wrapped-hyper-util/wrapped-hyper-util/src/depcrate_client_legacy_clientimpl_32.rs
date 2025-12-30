// Generated macro for impl_32 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_32 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_32"}
// Dependencies: {}
impl Client < () , () > { # [doc = " Create a builder to configure a new `Client`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"tokio\")]"] # [doc = " # fn run () {"] # [doc = " use std::time::Duration;"] # [doc = " use hyper_util::client::legacy::Client;"] # [doc = " use hyper_util::rt::{TokioExecutor, TokioTimer};"] # [doc = ""] # [doc = " let client = Client::builder(TokioExecutor::new())"] # [doc = "     .pool_timer(TokioTimer::new())"] # [doc = "     .pool_idle_timeout(Duration::from_secs(30))"] # [doc = "     .http2_only(true)"] # [doc = "     .build_http();"] # [doc = " # let infer: Client<_, http_body_util::Full<bytes::Bytes>> = client;"] # [doc = " # drop(infer);"] # [doc = " # }"] # [doc = " # fn main() {}"] # [doc = " ```"] pub fn builder < E > (executor : E) -> Builder where E : hyper :: rt :: Executor < BoxSendFuture > + Send + Sync + Clone + 'static , { Builder :: new (executor) } }
};
}
