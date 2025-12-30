// Generated macro for quiche_conn_timeout_as_millis (function)
macro_rules! Depcrate_ffiquiche_conn_timeout_as_millis {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_timeout_as_millis"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_timeout_as_millis (conn : & Connection) -> u64 { match conn . timeout () { Some (timeout) => timeout . as_millis () as u64 , None => u64 :: MAX , } }
};
}
