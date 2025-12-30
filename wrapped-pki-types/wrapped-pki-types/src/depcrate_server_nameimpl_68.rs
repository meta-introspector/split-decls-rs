// Generated macro for impl_68 (impl)
macro_rules! Depcrate_server_nameimpl_68 {
() => {
// Module: crate::server_name
// Provides: {"impl_68"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < std :: net :: Ipv6Addr > for ServerName < '_ > { fn from (v6 : std :: net :: Ipv6Addr) -> Self { Self :: IpAddress (IpAddr :: V6 (v6 . into ())) } }
};
}
