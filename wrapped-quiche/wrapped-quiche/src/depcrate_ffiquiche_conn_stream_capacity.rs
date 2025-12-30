// Generated macro for quiche_conn_stream_capacity (function)
macro_rules! Depcrate_ffiquiche_conn_stream_capacity {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_capacity"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_capacity (conn : & Connection , stream_id : u64 ,) -> ssize_t { match conn . stream_capacity (stream_id) { Ok (v) => v as ssize_t , Err (e) => e . to_c () , } }
};
}
