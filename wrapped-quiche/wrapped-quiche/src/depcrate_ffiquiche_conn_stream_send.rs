// Generated macro for quiche_conn_stream_send (function)
macro_rules! Depcrate_ffiquiche_conn_stream_send {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_send"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_send (conn : & mut Connection , stream_id : u64 , buf : * const u8 , buf_len : size_t , fin : bool , out_error_code : & mut u64 ,) -> ssize_t { if buf_len > < ssize_t > :: MAX as usize { panic ! ("The provided buffer is too large") ; } let buf = if buf . is_null () { assert_eq ! (buf_len , 0) ; & [] } else { unsafe { slice :: from_raw_parts (buf , buf_len) } } ; match conn . stream_send (stream_id , buf , fin) { Ok (v) => v as ssize_t , Err (e) => { match e { Error :: StreamReset (error) => * out_error_code = error , Error :: StreamStopped (error) => * out_error_code = error , _ => { } , } e . to_c () } , } }
};
}
