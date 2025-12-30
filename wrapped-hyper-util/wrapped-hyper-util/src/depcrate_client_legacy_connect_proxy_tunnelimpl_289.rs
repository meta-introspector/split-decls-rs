// Generated macro for impl_289 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_tunnelimpl_289 {
() => {
// Module: crate::client::legacy::connect::proxy::tunnel
// Provides: {"impl_289"}
// Dependencies: {}
impl std :: error :: Error for TunnelError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { TunnelError :: Io (ref e) => Some (e) , TunnelError :: ConnectFailed (ref e) => Some (& * * e) , _ => None , } } }
};
}
