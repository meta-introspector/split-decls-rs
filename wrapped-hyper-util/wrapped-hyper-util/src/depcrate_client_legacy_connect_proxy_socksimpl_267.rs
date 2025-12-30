// Generated macro for impl_267 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socksimpl_267 {
() => {
// Module: crate::client::legacy::connect::proxy::socks
// Provides: {"impl_267"}
// Dependencies: {}
impl < F , T , E > Future for Handshaking < F , T , E > where F : Future < Output = Result < T , E > > , { type Output = Result < T , SocksError < E > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . fut . poll (cx) } }
};
}
