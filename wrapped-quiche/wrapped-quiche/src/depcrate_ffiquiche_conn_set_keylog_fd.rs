// Generated macro for quiche_conn_set_keylog_fd (function)
macro_rules! Depcrate_ffiquiche_conn_set_keylog_fd {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_set_keylog_fd"}
// Dependencies: {}
# [no_mangle] # [cfg (unix)] pub extern "C" fn quiche_conn_set_keylog_fd (conn : & mut Connection , fd : c_int) { let f = unsafe { std :: fs :: File :: from_raw_fd (fd) } ; let writer = std :: io :: BufWriter :: new (f) ; conn . set_keylog (Box :: new (writer)) ; }
};
}
