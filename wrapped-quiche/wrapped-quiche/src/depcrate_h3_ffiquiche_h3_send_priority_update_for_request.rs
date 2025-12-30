// Generated macro for quiche_h3_send_priority_update_for_request (function)
macro_rules! Depcrate_h3_ffiquiche_h3_send_priority_update_for_request {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_send_priority_update_for_request"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_send_priority_update_for_request (conn : & mut h3 :: Connection , quic_conn : & mut Connection , stream_id : u64 , priority : & Priority ,) -> c_int { match conn . send_priority_update_for_request (quic_conn , stream_id , priority) { Ok (()) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
