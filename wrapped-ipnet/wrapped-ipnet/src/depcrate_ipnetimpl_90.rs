// Generated macro for impl_90 (impl)
macro_rules! Depcrate_ipnetimpl_90 {
() => {
// Module: crate::ipnet
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a > Contains < & 'a Ipv4Addr > for Ipv4Net { fn contains (& self , other : & 'a Ipv4Addr) -> bool { self . network () <= * other && * other <= self . broadcast () } }
};
}
