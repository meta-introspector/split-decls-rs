// Generated macro for quiche_connection_id_iter_free (function)
macro_rules! Depcrate_ffiquiche_connection_id_iter_free {
() => {
// Module: crate::ffi
// Provides: {"quiche_connection_id_iter_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_connection_id_iter_free (iter : * mut ConnectionIdIter) { drop (unsafe { Box :: from_raw (iter) }) ; }
};
}
