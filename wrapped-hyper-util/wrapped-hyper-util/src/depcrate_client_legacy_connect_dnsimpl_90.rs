// Generated macro for impl_90 (impl)
macro_rules! Depcrate_client_legacy_connect_dnsimpl_90 {
() => {
// Module: crate::client::legacy::connect::dns
// Provides: {"impl_90"}
// Dependencies: {}
impl Service < Name > for GaiResolver { type Response = GaiAddrs ; type Error = io :: Error ; type Future = GaiFuture ; fn poll_ready (& mut self , _cx : & mut task :: Context < '_ >) -> Poll < Result < () , io :: Error > > { Poll :: Ready (Ok (())) } fn call (& mut self , name : Name) -> Self :: Future { let blocking = tokio :: task :: spawn_blocking (move | | { (& * name . host , 0) . to_socket_addrs () . map (| i | SocketAddrs { iter : i }) }) ; GaiFuture { inner : blocking } } }
};
}
