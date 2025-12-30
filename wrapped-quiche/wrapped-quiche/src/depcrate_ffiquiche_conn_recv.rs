// Generated macro for quiche_conn_recv (function)
macro_rules! Depcrate_ffiquiche_conn_recv {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_recv"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_recv (conn : & mut Connection , buf : * mut u8 , buf_len : size_t , info : & RecvInfo ,) -> ssize_t { if buf_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let buf = unsafe { slice :: from_raw_parts_mut (buf , buf_len) } ; match conn . recv (buf , info . into ()) { Ok (v) => v as ssize_t , Err (e) => e . to_c () , } }
};
}
