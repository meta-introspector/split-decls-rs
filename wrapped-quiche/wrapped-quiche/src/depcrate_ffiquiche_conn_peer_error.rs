// Generated macro for quiche_conn_peer_error (function)
macro_rules! Depcrate_ffiquiche_conn_peer_error {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_peer_error"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_peer_error (conn : & Connection , is_app : * mut bool , error_code : * mut u64 , reason : & mut * const u8 , reason_len : & mut size_t ,) -> bool { match & conn . peer_error { Some (conn_err) => unsafe { * is_app = conn_err . is_app ; * error_code = conn_err . error_code ; * reason = conn_err . reason . as_ptr () ; * reason_len = conn_err . reason . len () ; true } , None => false , } }
};
}
