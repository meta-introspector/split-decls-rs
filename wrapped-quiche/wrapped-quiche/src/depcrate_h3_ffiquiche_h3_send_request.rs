// Generated macro for quiche_h3_send_request (function)
macro_rules! Depcrate_h3_ffiquiche_h3_send_request {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_send_request"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_send_request (conn : & mut h3 :: Connection , quic_conn : & mut Connection , headers : * const Header , headers_len : size_t , fin : bool ,) -> i64 { let req_headers = headers_from_ptr (headers , headers_len) ; match conn . send_request (quic_conn , & req_headers , fin) { Ok (v) => v as i64 , Err (e) => e . to_c () as i64 , } }
};
}
