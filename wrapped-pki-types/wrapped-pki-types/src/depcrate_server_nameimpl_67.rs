// Generated macro for impl_67 (impl)
macro_rules! Depcrate_server_nameimpl_67 {
() => {
// Module: crate::server_name
// Provides: {"impl_67"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < std :: net :: Ipv4Addr > for ServerName < '_ > { fn from (v4 : std :: net :: Ipv4Addr) -> Self { Self :: IpAddress (IpAddr :: V4 (v4 . into ())) } }
};
}
