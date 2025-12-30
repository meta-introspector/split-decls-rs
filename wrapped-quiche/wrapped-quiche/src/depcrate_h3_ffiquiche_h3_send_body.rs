// Generated macro for quiche_h3_send_body (function)
macro_rules! Depcrate_h3_ffiquiche_h3_send_body {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_send_body"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_send_body (conn : & mut h3 :: Connection , quic_conn : & mut Connection , stream_id : u64 , body : * const u8 , body_len : size_t , fin : bool ,) -> ssize_t { if body_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let body = unsafe { slice :: from_raw_parts (body , body_len) } ; match conn . send_body (quic_conn , stream_id , body , fin) { Ok (v) => v as ssize_t , Err (e) => e . to_c () , } }
};
}
