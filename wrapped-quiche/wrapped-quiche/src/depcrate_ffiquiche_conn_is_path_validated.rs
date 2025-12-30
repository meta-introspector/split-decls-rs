// Generated macro for quiche_conn_is_path_validated (function)
macro_rules! Depcrate_ffiquiche_conn_is_path_validated {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_is_path_validated"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_is_path_validated (conn : & Connection , from : & sockaddr , from_len : socklen_t , to : & sockaddr , to_len : socklen_t ,) -> c_int { let from = std_addr_from_c (from , from_len) ; let to = std_addr_from_c (to , to_len) ; match conn . is_path_validated (from , to) { Ok (v) => v as c_int , Err (e) => e . to_c () as c_int , } }
};
}
