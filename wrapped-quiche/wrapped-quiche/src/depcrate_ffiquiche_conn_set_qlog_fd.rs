// Generated macro for quiche_conn_set_qlog_fd (function)
macro_rules! Depcrate_ffiquiche_conn_set_qlog_fd {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_set_qlog_fd"}
// Dependencies: {}
# [no_mangle] # [cfg (all (unix , feature = "qlog"))] pub extern "C" fn quiche_conn_set_qlog_fd (conn : & mut Connection , fd : c_int , log_title : * const c_char , log_desc : * const c_char ,) { let f = unsafe { std :: fs :: File :: from_raw_fd (fd) } ; let writer = std :: io :: BufWriter :: new (f) ; let title = unsafe { ffi :: CStr :: from_ptr (log_title) . to_str () . unwrap () } ; let description = unsafe { ffi :: CStr :: from_ptr (log_desc) . to_str () . unwrap () } ; conn . set_qlog (Box :: new (writer) , title . to_string () , format ! ("{} id={}" , description , conn . trace_id) ,) ; }
};
}
