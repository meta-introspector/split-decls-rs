// Generated macro for Client (struct)
macro_rules! Depcrate_client_legacy_clientClient {
() => {
// Module: crate::client::legacy::client
// Provides: {"Client"}
// Dependencies: {}
# [doc = " A Client to make outgoing HTTP requests."] # [doc = ""] # [doc = " `Client` is cheap to clone and cloning is the recommended way to share a `Client`. The"] # [doc = " underlying connection pool will be reused."] # [cfg_attr (docsrs , doc (cfg (any (feature = "http1" , feature = "http2"))))] pub struct Client < C , B > { config : Config , connector : C , exec : Exec , # [cfg (feature = "http1")] h1_builder : hyper :: client :: conn :: http1 :: Builder , # [cfg (feature = "http2")] h2_builder : hyper :: client :: conn :: http2 :: Builder < Exec > , pool : pool :: Pool < PoolClient < B > , PoolKey > , }
};
}
