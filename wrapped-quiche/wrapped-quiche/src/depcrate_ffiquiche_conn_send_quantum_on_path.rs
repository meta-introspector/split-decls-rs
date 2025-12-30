// Generated macro for quiche_conn_send_quantum_on_path (function)
macro_rules! Depcrate_ffiquiche_conn_send_quantum_on_path {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_send_quantum_on_path"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_send_quantum_on_path (conn : & Connection , local : & sockaddr , local_len : socklen_t , peer : & sockaddr , peer_len : socklen_t ,) -> size_t { let local = std_addr_from_c (local , local_len) ; let peer = std_addr_from_c (peer , peer_len) ; conn . send_quantum_on_path (local , peer) as size_t }
};
}
