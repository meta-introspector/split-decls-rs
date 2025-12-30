// Generated macro for set_port (function)
macro_rules! Depcrate_client_legacy_connect_httpset_port {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"set_port"}
// Dependencies: {}
# [doc = " Respect explicit ports in the URI, if none, either"] # [doc = " keep non `0` ports resolved from a custom dns resolver,"] # [doc = " or use the default port for the scheme."] fn set_port (addr : & mut SocketAddr , host_port : u16 , explicit : bool) { if explicit || addr . port () == 0 { addr . set_port (host_port) } ; }
};
}
