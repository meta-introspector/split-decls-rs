// Generated macro for quiche_conn_timeout_as_nanos (function)
macro_rules! Depcrate_ffiquiche_conn_timeout_as_nanos {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_timeout_as_nanos"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_timeout_as_nanos (conn : & Connection) -> u64 { match conn . timeout () { Some (timeout) => timeout . as_nanos () as u64 , None => u64 :: MAX , } }
};
}
