// Generated macro for impl_92 (impl)
macro_rules! Depcrate_ipnetimpl_92 {
() => {
// Module: crate::ipnet
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'a > Contains < & 'a Ipv6Addr > for Ipv6Net { fn contains (& self , other : & 'a Ipv6Addr) -> bool { self . network () <= * other && * other <= self . broadcast () } }
};
}
