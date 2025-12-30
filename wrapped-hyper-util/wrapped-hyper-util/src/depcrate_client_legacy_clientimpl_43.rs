// Generated macro for impl_43 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_43 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_43"}
// Dependencies: {}
impl < B > PoolClient < B > { fn poll_ready (& mut self , # [allow (unused_variables)] cx : & mut task :: Context < '_ > ,) -> Poll < Result < () , Error > > { match self . tx { # [cfg (feature = "http1")] PoolTx :: Http1 (ref mut tx) => tx . poll_ready (cx) . map_err (Error :: closed) , # [cfg (feature = "http2")] PoolTx :: Http2 (_) => Poll :: Ready (Ok (())) , } } fn is_http1 (& self) -> bool { ! self . is_http2 () } fn is_http2 (& self) -> bool { match self . tx { # [cfg (feature = "http1")] PoolTx :: Http1 (_) => false , # [cfg (feature = "http2")] PoolTx :: Http2 (_) => true , } } fn is_poisoned (& self) -> bool { self . conn_info . poisoned . poisoned () } fn is_ready (& self) -> bool { match self . tx { # [cfg (feature = "http1")] PoolTx :: Http1 (ref tx) => tx . is_ready () , # [cfg (feature = "http2")] PoolTx :: Http2 (ref tx) => tx . is_ready () , } } }
};
}
