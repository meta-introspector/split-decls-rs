// Generated macro for quiche_conn_set_session (function)
macro_rules! Depcrate_ffiquiche_conn_set_session {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_set_session"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_set_session (conn : & mut Connection , buf : * const u8 , buf_len : size_t ,) -> c_int { let buf = unsafe { slice :: from_raw_parts (buf , buf_len) } ; match conn . set_session (buf) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
