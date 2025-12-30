// Generated macro for impl_157 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_157 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_157"}
// Dependencies: {}
impl ConnectingTcpRemote { fn new (addrs : dns :: SocketAddrs , connect_timeout : Option < Duration >) -> Self { let connect_timeout = connect_timeout . and_then (| t | t . checked_div (addrs . len () as u32)) ; Self { addrs , connect_timeout , } } }
};
}
