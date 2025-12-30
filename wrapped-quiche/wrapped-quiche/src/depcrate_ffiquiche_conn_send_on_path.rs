// Generated macro for quiche_conn_send_on_path (function)
macro_rules! Depcrate_ffiquiche_conn_send_on_path {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_send_on_path"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_send_on_path (conn : & mut Connection , out : * mut u8 , out_len : size_t , from : * const sockaddr , from_len : socklen_t , to : * const sockaddr , to_len : socklen_t , out_info : & mut SendInfo ,) -> ssize_t { if out_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let from = optional_std_addr_from_c (from , from_len) ; let to = optional_std_addr_from_c (to , to_len) ; let out = unsafe { slice :: from_raw_parts_mut (out , out_len) } ; match conn . send_on_path (out , from , to) { Ok ((v , info)) => { out_info . from_len = std_addr_to_c (& info . from , & mut out_info . from) ; out_info . to_len = std_addr_to_c (& info . to , & mut out_info . to) ; std_time_to_c (& info . at , & mut out_info . at) ; v as ssize_t } , Err (e) => e . to_c () , } }
};
}
