// Generated macro for quiche_conn_dgram_recv (function)
macro_rules! Depcrate_ffiquiche_conn_dgram_recv {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_dgram_recv"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_dgram_recv (conn : & mut Connection , out : * mut u8 , out_len : size_t ,) -> ssize_t { if out_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let out = unsafe { slice :: from_raw_parts_mut (out , out_len) } ; let out_len = match conn . dgram_recv (out) { Ok (v) => v , Err (e) => return e . to_c () , } ; out_len as ssize_t }
};
}
