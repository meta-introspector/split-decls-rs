// Generated macro for quiche_conn_stream_writable (function)
macro_rules! Depcrate_ffiquiche_conn_stream_writable {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_writable"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_writable (conn : & mut Connection , stream_id : u64 , len : usize ,) -> c_int { match conn . stream_writable (stream_id , len) { Ok (true) => 1 , Ok (false) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
