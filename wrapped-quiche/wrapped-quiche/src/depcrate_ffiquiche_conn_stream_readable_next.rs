// Generated macro for quiche_conn_stream_readable_next (function)
macro_rules! Depcrate_ffiquiche_conn_stream_readable_next {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_readable_next"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_readable_next (conn : & mut Connection) -> i64 { conn . stream_readable_next () . map (| v | v as i64) . unwrap_or (- 1) }
};
}
