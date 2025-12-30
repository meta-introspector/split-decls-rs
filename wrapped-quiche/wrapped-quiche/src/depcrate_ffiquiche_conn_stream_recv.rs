// Generated macro for quiche_conn_stream_recv (function)
macro_rules! Depcrate_ffiquiche_conn_stream_recv {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_recv"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_recv (conn : & mut Connection , stream_id : u64 , out : * mut u8 , out_len : size_t , fin : & mut bool , out_error_code : & mut u64 ,) -> ssize_t { if out_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let out = unsafe { slice :: from_raw_parts_mut (out , out_len) } ; let (out_len , out_fin) = match conn . stream_recv (stream_id , out) { Ok (v) => v , Err (e) => { match e { Error :: StreamReset (error) => * out_error_code = error , Error :: StreamStopped (error) => * out_error_code = error , _ => { } , } return e . to_c () ; } , } ; * fin = out_fin ; out_len as ssize_t }
};
}
