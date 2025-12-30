// Generated macro for quiche_conn_send_ack_eliciting (function)
macro_rules! Depcrate_ffiquiche_conn_send_ack_eliciting {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_send_ack_eliciting"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_send_ack_eliciting (conn : & mut Connection ,) -> ssize_t { match conn . send_ack_eliciting () { Ok (()) => 0 , Err (e) => e . to_c () , } }
};
}
