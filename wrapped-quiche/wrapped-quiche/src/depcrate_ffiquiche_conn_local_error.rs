// Generated macro for quiche_conn_local_error (function)
macro_rules! Depcrate_ffiquiche_conn_local_error {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_local_error"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_local_error (conn : & Connection , is_app : * mut bool , error_code : * mut u64 , reason : & mut * const u8 , reason_len : & mut size_t ,) -> bool { match & conn . local_error { Some (conn_err) => unsafe { * is_app = conn_err . is_app ; * error_code = conn_err . error_code ; * reason = conn_err . reason . as_ptr () ; * reason_len = conn_err . reason . len () ; true } , None => false , } }
};
}
