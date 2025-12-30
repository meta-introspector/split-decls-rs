// Generated macro for get_session_bytes (function)
macro_rules! Depcrate_tls_boringsslget_session_bytes {
() => {
// Module: crate::tls::boringssl
// Provides: {"get_session_bytes"}
// Dependencies: {}
pub (super) fn get_session_bytes (session : * mut SSL_SESSION) -> Result < Vec < u8 > > { let session_bytes = unsafe { let mut out : * mut u8 = ptr :: null_mut () ; let mut out_len : usize = 0 ; if SSL_SESSION_to_bytes (session , & mut out , & mut out_len) == 0 { return Err (Error :: TlsFail) ; } let session_bytes = slice :: from_raw_parts (out , out_len) . to_vec () ; OPENSSL_free (out as * mut c_void) ; session_bytes } ; Ok (session_bytes) }
};
}
