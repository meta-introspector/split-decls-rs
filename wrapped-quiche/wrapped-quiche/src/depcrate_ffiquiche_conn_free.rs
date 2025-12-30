// Generated macro for quiche_conn_free (function)
macro_rules! Depcrate_ffiquiche_conn_free {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_free (conn : * mut Connection) { drop (unsafe { Box :: from_raw (conn) }) ; }
};
}
