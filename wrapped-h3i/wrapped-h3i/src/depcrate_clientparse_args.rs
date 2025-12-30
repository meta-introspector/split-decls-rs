// Generated macro for parse_args (function)
macro_rules! Depcrate_clientparse_args {
() => {
// Module: crate::client
// Provides: {"parse_args"}
// Dependencies: {}
pub (crate) fn parse_args (args : & Config) -> ParsedArgs < '_ > { let connect_url = if ! args . omit_sni { args . host_port . split (':') . next () } else { None } ; let (peer_addr , bind_addr) = resolve_socket_addrs (args) ; ParsedArgs { peer_addr , bind_addr , connect_url , } }
};
}
