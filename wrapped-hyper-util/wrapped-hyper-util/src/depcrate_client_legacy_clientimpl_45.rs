// Generated macro for impl_45 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_45 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_45"}
// Dependencies: {}
impl < B > pool :: Poolable for PoolClient < B > where B : Send + 'static , { fn is_open (& self) -> bool { ! self . is_poisoned () && self . is_ready () } fn reserve (self) -> pool :: Reservation < Self > { match self . tx { # [cfg (feature = "http1")] PoolTx :: Http1 (tx) => pool :: Reservation :: Unique (PoolClient { conn_info : self . conn_info , tx : PoolTx :: Http1 (tx) , }) , # [cfg (feature = "http2")] PoolTx :: Http2 (tx) => { let b = PoolClient { conn_info : self . conn_info . clone () , tx : PoolTx :: Http2 (tx . clone ()) , } ; let a = PoolClient { conn_info : self . conn_info , tx : PoolTx :: Http2 (tx) , } ; pool :: Reservation :: Shared (a , b) } } } fn can_share (& self) -> bool { self . is_http2 () } }
};
}
