// Generated macro for get_session_bytes (function)
macro_rules! Depcrate_tls_openssl_quictlsget_session_bytes {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"get_session_bytes"}
// Dependencies: {}
pub (super) fn get_session_bytes (session : * mut SSL_SESSION) -> Result < Vec < u8 > > { let session_bytes = unsafe { let out_len = i2d_SSL_SESSION (session , ptr :: null_mut ()) ; if out_len == 0 { return Err (Error :: TlsFail) ; } let mut out : Vec < c_uchar > = Vec :: with_capacity (out_len as usize) ; let out_len = i2d_SSL_SESSION (session , & mut out . as_mut_ptr ()) ; let session_bytes = slice :: from_raw_parts (out . as_mut_ptr () , out_len as usize) . to_vec () ; session_bytes } ; Ok (session_bytes) }
};
}
