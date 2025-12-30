// Generated macro for impl_409 (impl)
macro_rules! Depcrate_client_proxy_matcherimpl_409 {
() => {
// Module: crate::client::proxy::matcher
// Provides: {"impl_409"}
// Dependencies: {}
impl IpMatcher { fn contains (& self , addr : IpAddr) -> bool { for ip in & self . 0 { match ip { Ip :: Address (address) => { if & addr == address { return true ; } } Ip :: Network (net) => { if net . contains (& addr) { return true ; } } } } false } }
};
}
