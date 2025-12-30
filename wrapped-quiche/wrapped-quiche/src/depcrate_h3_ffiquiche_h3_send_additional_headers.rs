// Generated macro for quiche_h3_send_additional_headers (function)
macro_rules! Depcrate_h3_ffiquiche_h3_send_additional_headers {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_send_additional_headers"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_send_additional_headers (conn : & mut h3 :: Connection , quic_conn : & mut Connection , stream_id : u64 , headers : * const Header , headers_len : size_t , is_trailer_section : bool , fin : bool ,) -> c_int { let headers = headers_from_ptr (headers , headers_len) ; match conn . send_additional_headers (quic_conn , stream_id , & headers , is_trailer_section , fin ,) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
