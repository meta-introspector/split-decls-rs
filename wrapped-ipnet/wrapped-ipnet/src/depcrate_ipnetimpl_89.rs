// Generated macro for impl_89 (impl)
macro_rules! Depcrate_ipnetimpl_89 {
() => {
// Module: crate::ipnet
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'a > Contains < & 'a Ipv4Net > for Ipv4Net { fn contains (& self , other : & 'a Ipv4Net) -> bool { self . network () <= other . network () && other . broadcast () <= self . broadcast () } }
};
}
