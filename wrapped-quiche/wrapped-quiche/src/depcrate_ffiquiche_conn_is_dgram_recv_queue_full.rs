// Generated macro for quiche_conn_is_dgram_recv_queue_full (function)
macro_rules! Depcrate_ffiquiche_conn_is_dgram_recv_queue_full {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_is_dgram_recv_queue_full"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_is_dgram_recv_queue_full (conn : & Connection ,) -> bool { conn . is_dgram_recv_queue_full () }
};
}
