// Generated macro for quiche_conn_set_qlog_path (function)
macro_rules! Depcrate_ffiquiche_conn_set_qlog_path {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_set_qlog_path"}
// Dependencies: {}
# [no_mangle] # [cfg (feature = "qlog")] pub extern "C" fn quiche_conn_set_qlog_path (conn : & mut Connection , path : * const c_char , log_title : * const c_char , log_desc : * const c_char ,) -> bool { let filename = unsafe { ffi :: CStr :: from_ptr (path) . to_str () . unwrap () } ; let file = std :: fs :: OpenOptions :: new () . write (true) . create_new (true) . open (filename) ; let writer = match file { Ok (f) => std :: io :: BufWriter :: new (f) , Err (_) => return false , } ; let title = unsafe { ffi :: CStr :: from_ptr (log_title) . to_str () . unwrap () } ; let description = unsafe { ffi :: CStr :: from_ptr (log_desc) . to_str () . unwrap () } ; conn . set_qlog (Box :: new (writer) , title . to_string () , format ! ("{} id={}" , description , conn . trace_id) ,) ; true }
};
}
