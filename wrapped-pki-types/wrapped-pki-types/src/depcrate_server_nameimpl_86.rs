// Generated macro for impl_86 (impl)
macro_rules! Depcrate_server_nameimpl_86 {
() => {
// Module: crate::server_name
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < std :: net :: IpAddr > for IpAddr { fn from (addr : std :: net :: IpAddr) -> Self { match addr { std :: net :: IpAddr :: V4 (v4) => Self :: V4 (v4 . into ()) , std :: net :: IpAddr :: V6 (v6) => Self :: V6 (v6 . into ()) , } } }
};
}
