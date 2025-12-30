// Generated macro for quiche_h3_send_goaway (function)
macro_rules! Depcrate_h3_ffiquiche_h3_send_goaway {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_send_goaway"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_send_goaway (conn : & mut h3 :: Connection , quic_conn : & mut Connection , id : u64 ,) -> c_int { match conn . send_goaway (quic_conn , id) { Ok (()) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
