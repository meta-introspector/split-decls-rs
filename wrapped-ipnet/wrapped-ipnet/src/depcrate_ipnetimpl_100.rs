// Generated macro for impl_100 (impl)
macro_rules! Depcrate_ipnetimpl_100 {
() => {
// Module: crate::ipnet
// Provides: {"impl_100"}
// Dependencies: {}
impl Iterator for IpSubnets { type Item = IpNet ; fn next (& mut self) -> Option < Self :: Item > { match * self { IpSubnets :: V4 (ref mut a) => a . next () . map (IpNet :: V4) , IpSubnets :: V6 (ref mut a) => a . next () . map (IpNet :: V6) , } } }
};
}
