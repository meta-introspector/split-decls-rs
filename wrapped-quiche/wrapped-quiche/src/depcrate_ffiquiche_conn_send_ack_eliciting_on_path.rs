// Generated macro for quiche_conn_send_ack_eliciting_on_path (function)
macro_rules! Depcrate_ffiquiche_conn_send_ack_eliciting_on_path {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_send_ack_eliciting_on_path"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_send_ack_eliciting_on_path (conn : & mut Connection , local : & sockaddr , local_len : socklen_t , peer : & sockaddr , peer_len : socklen_t ,) -> ssize_t { let local = std_addr_from_c (local , local_len) ; let peer = std_addr_from_c (peer , peer_len) ; match conn . send_ack_eliciting_on_path (local , peer) { Ok (()) => 0 , Err (e) => e . to_c () , } }
};
}
