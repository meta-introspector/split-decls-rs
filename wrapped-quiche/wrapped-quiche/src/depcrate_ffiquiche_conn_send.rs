// Generated macro for quiche_conn_send (function)
macro_rules! Depcrate_ffiquiche_conn_send {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_send"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_send (conn : & mut Connection , out : * mut u8 , out_len : size_t , out_info : & mut SendInfo ,) -> ssize_t { if out_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let out = unsafe { slice :: from_raw_parts_mut (out , out_len) } ; match conn . send (out) { Ok ((v , info)) => { out_info . from_len = std_addr_to_c (& info . from , & mut out_info . from) ; out_info . to_len = std_addr_to_c (& info . to , & mut out_info . to) ; std_time_to_c (& info . at , & mut out_info . at) ; v as ssize_t } , Err (e) => e . to_c () , } }
};
}
