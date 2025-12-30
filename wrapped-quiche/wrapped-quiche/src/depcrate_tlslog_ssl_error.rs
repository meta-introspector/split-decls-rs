// Generated macro for log_ssl_error (function)
macro_rules! Depcrate_tlslog_ssl_error {
() => {
// Module: crate::tls
// Provides: {"log_ssl_error"}
// Dependencies: {}
fn log_ssl_error () { let mut err = [0u8 ; 1024] ; unsafe { let e = ERR_peek_error () ; ERR_error_string_n (e , err . as_mut_ptr () as * mut c_char , err . len ()) ; } let cstr = ffi :: CStr :: from_bytes_until_nul (& err) . expect ("ERR_error_string_n should write a null terminated string") ; trace ! ("{}" , cstr . to_str () . expect ("ERR_error_string_n should create a valid UTF-8 message")) ; }
};
}
