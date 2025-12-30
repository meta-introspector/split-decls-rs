// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl From < Ipv4Addr > for IpAddrKey { fn from (addr : Ipv4Addr) -> Self { Self (addr . to_ipv6_mapped ()) } }
};
}
