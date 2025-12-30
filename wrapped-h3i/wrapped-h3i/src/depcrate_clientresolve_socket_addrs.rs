// Generated macro for resolve_socket_addrs (function)
macro_rules! Depcrate_clientresolve_socket_addrs {
() => {
// Module: crate::client
// Provides: {"resolve_socket_addrs"}
// Dependencies: {}
fn resolve_socket_addrs (args : & Config) -> (SocketAddr , SocketAddr) { let peer_addr = if let Some (addr) = & args . connect_to { addr . parse () . expect ("--connect-to is expected to be a string containing an IPv4 or IPv6 address with a port. E.g. 192.0.2.0:443") } else { let x = format ! ("https://{}" , args . host_port) ; * url :: Url :: parse (& x) . unwrap () . socket_addrs (| | None) . unwrap () . first () . unwrap () } ; let bind_addr = match peer_addr { std :: net :: SocketAddr :: V4 (_) => format ! ("0.0.0.0:{}" , args . source_port) , std :: net :: SocketAddr :: V6 (_) => format ! ("[::]:{}" , args . source_port) , } ; (peer_addr , bind_addr . parse () . expect ("unable to parse bind address") ,) }
};
}
