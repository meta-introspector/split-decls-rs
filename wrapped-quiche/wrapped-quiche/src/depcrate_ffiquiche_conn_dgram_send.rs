// Generated macro for quiche_conn_dgram_send (function)
macro_rules! Depcrate_ffiquiche_conn_dgram_send {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_dgram_send"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_dgram_send (conn : & mut Connection , buf : * const u8 , buf_len : size_t ,) -> ssize_t { if buf_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let buf = unsafe { slice :: from_raw_parts (buf , buf_len) } ; match conn . dgram_send (buf) { Ok (_) => buf_len as ssize_t , Err (e) => e . to_c () , } }
};
}
