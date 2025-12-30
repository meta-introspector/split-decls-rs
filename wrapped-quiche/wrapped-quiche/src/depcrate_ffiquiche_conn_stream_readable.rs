// Generated macro for quiche_conn_stream_readable (function)
macro_rules! Depcrate_ffiquiche_conn_stream_readable {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_readable"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_readable (conn : & Connection , stream_id : u64 ,) -> bool { conn . stream_readable (stream_id) }
};
}
