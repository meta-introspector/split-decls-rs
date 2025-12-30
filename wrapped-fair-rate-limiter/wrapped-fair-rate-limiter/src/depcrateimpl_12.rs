// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl IpAddrKey { # [must_use] pub fn new (ip_addr : IpAddr) -> Self { match ip_addr { IpAddr :: V4 (addr) => Self (addr . to_ipv6_mapped ()) , IpAddr :: V6 (addr) => Self (addr) , } } }
};
}
