// Generated macro for quiche_conn_writable (function)
macro_rules! Depcrate_ffiquiche_conn_writable {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_writable"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_writable (conn : & Connection) -> * mut StreamIter { Box :: into_raw (Box :: new (conn . writable ())) }
};
}
