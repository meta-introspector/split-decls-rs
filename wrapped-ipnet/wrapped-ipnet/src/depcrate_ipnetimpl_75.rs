// Generated macro for impl_75 (impl)
macro_rules! Depcrate_ipnetimpl_75 {
() => {
// Module: crate::ipnet
// Provides: {"impl_75"}
// Dependencies: {}
impl From < IpAddr > for IpNet { fn from (addr : IpAddr) -> IpNet { match addr { IpAddr :: V4 (a) => IpNet :: V4 (a . into ()) , IpAddr :: V6 (a) => IpNet :: V6 (a . into ()) , } } }
};
}
