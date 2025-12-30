// Generated macro for impl_87 (impl)
macro_rules! Depcrate_server_nameimpl_87 {
() => {
// Module: crate::server_name
// Provides: {"impl_87"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < IpAddr > for std :: net :: IpAddr { fn from (value : IpAddr) -> Self { match value { IpAddr :: V4 (v4) => Self :: from (std :: net :: Ipv4Addr :: from (v4)) , IpAddr :: V6 (v6) => Self :: from (std :: net :: Ipv6Addr :: from (v6)) , } } }
};
}
