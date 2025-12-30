// Generated macro for quiche_h3_send_response (function)
macro_rules! Depcrate_h3_ffiquiche_h3_send_response {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_send_response"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_send_response (conn : & mut h3 :: Connection , quic_conn : & mut Connection , stream_id : u64 , headers : * const Header , headers_len : size_t , fin : bool ,) -> c_int { let resp_headers = headers_from_ptr (headers , headers_len) ; match conn . send_response (quic_conn , stream_id , & resp_headers , fin) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
