// Generated macro for quiche_conn_set_keylog_path (function)
macro_rules! Depcrate_ffiquiche_conn_set_keylog_path {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_set_keylog_path"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_set_keylog_path (conn : & mut Connection , path : * const c_char ,) -> bool { let filename = unsafe { ffi :: CStr :: from_ptr (path) . to_str () . unwrap () } ; let file = std :: fs :: OpenOptions :: new () . create (true) . append (true) . open (filename) ; let writer = match file { Ok (f) => std :: io :: BufWriter :: new (f) , Err (_) => return false , } ; conn . set_keylog (Box :: new (writer)) ; true }
};
}
