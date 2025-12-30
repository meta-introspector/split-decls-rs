// Generated macro for quiche_conn_dgram_send_queue_byte_size (function)
macro_rules! Depcrate_ffiquiche_conn_dgram_send_queue_byte_size {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_dgram_send_queue_byte_size"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_dgram_send_queue_byte_size (conn : & Connection ,) -> ssize_t { conn . dgram_send_queue_byte_size () as ssize_t }
};
}
