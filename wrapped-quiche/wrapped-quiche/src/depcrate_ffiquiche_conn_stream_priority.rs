// Generated macro for quiche_conn_stream_priority (function)
macro_rules! Depcrate_ffiquiche_conn_stream_priority {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_stream_priority"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_stream_priority (conn : & mut Connection , stream_id : u64 , urgency : u8 , incremental : bool ,) -> c_int { match conn . stream_priority (stream_id , urgency , incremental) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
