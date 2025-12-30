// Generated macro for quiche_h3_conn_free (function)
macro_rules! Depcrate_h3_ffiquiche_h3_conn_free {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_conn_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_conn_free (conn : * mut h3 :: Connection) { drop (unsafe { Box :: from_raw (conn) }) ; }
};
}
