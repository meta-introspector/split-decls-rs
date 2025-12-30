// Generated macro for quiche_conn_dgram_recv_queue_len (function)
macro_rules! Depcrate_ffiquiche_conn_dgram_recv_queue_len {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_dgram_recv_queue_len"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_dgram_recv_queue_len (conn : & Connection) -> ssize_t { conn . dgram_recv_queue_len () as ssize_t }
};
}
