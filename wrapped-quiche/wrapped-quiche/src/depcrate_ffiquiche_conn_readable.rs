// Generated macro for quiche_conn_readable (function)
macro_rules! Depcrate_ffiquiche_conn_readable {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_readable"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_readable (conn : & Connection) -> * mut StreamIter { Box :: into_raw (Box :: new (conn . readable ())) }
};
}
