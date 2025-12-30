// Generated macro for Builder (struct)
macro_rules! Depcrate_client_legacy_clientBuilder {
() => {
// Module: crate::client::legacy::client
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder to configure a new [`Client`](Client)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"tokio\")]"] # [doc = " # fn run () {"] # [doc = " use std::time::Duration;"] # [doc = " use hyper_util::client::legacy::Client;"] # [doc = " use hyper_util::rt::TokioExecutor;"] # [doc = ""] # [doc = " let client = Client::builder(TokioExecutor::new())"] # [doc = "     .pool_idle_timeout(Duration::from_secs(30))"] # [doc = "     .http2_only(true)"] # [doc = "     .build_http();"] # [doc = " # let infer: Client<_, http_body_util::Full<bytes::Bytes>> = client;"] # [doc = " # drop(infer);"] # [doc = " # }"] # [doc = " # fn main() {}"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (any (feature = "http1" , feature = "http2"))))] # [derive (Clone)] pub struct Builder { client_config : Config , exec : Exec , # [cfg (feature = "http1")] h1_builder : hyper :: client :: conn :: http1 :: Builder , # [cfg (feature = "http2")] h2_builder : hyper :: client :: conn :: http2 :: Builder < Exec > , pool_config : pool :: Config , pool_timer : Option < timer :: Timer > , }
};
}
