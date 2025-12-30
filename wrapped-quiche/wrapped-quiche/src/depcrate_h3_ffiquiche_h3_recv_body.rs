// Generated macro for quiche_h3_recv_body (function)
macro_rules! Depcrate_h3_ffiquiche_h3_recv_body {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_recv_body"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_recv_body (conn : & mut h3 :: Connection , quic_conn : & mut Connection , stream_id : u64 , out : * mut u8 , out_len : size_t ,) -> ssize_t { if out_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let out = unsafe { slice :: from_raw_parts_mut (out , out_len) } ; match conn . recv_body (quic_conn , stream_id , out) { Ok (v) => v as ssize_t , Err (e) => e . to_c () , } }
};
}
