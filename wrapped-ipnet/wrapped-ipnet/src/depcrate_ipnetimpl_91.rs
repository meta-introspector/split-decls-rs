// Generated macro for impl_91 (impl)
macro_rules! Depcrate_ipnetimpl_91 {
() => {
// Module: crate::ipnet
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'a > Contains < & 'a Ipv6Net > for Ipv6Net { fn contains (& self , other : & 'a Ipv6Net) -> bool { self . network () <= other . network () && other . broadcast () <= self . broadcast () } }
};
}
