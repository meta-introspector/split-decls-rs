// Generated macro for quiche_conn_stream_finished (function)
macro_rules! Depcrate_ffiquiche_conn_stream_finished {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_finished"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_finished (conn : & Connection , stream_id : u64 ,) -> bool { conn . stream_finished (stream_id) }
};
}
