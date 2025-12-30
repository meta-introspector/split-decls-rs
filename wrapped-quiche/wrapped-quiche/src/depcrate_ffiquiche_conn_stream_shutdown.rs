// Generated macro for quiche_conn_stream_shutdown (function)
macro_rules! Depcrate_ffiquiche_conn_stream_shutdown {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_shutdown"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_shutdown (conn : & mut Connection , stream_id : u64 , direction : Shutdown , err : u64 ,) -> c_int { match conn . stream_shutdown (stream_id , direction , err) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
