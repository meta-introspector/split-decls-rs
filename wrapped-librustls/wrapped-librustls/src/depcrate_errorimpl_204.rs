// Generated macro for impl_204 (impl)
macro_rules! Depcrate_errorimpl_204 {
() => {
// Module: crate::error
// Provides: {"impl_204"}
// Dependencies: {}
impl rustls_result { # [doc = " After a rustls function returns an error, you may call"] # [doc = " this to get a pointer to a buffer containing a detailed error"] # [doc = " message."] # [doc = ""] # [doc = " The contents of the error buffer will be out_n bytes long,"] # [doc = " UTF-8 encoded, and not NUL-terminated."] # [no_mangle] pub extern "C" fn rustls_error (result : c_uint , buf : * mut c_char , len : size_t , out_n : * mut size_t ,) { ffi_panic_boundary ! { if buf . is_null () { return ; } if out_n . is_null () { return ; } let error_str = rustls_result :: from (result) . to_string () ; let out_len = min (len , error_str . len ()) ; unsafe { std :: ptr :: copy_nonoverlapping (error_str . as_ptr () as * mut c_char , buf , out_len) ; * out_n = out_len ; } } } # [doc = " Returns true if the `result` is a certificate related error."] # [no_mangle] pub extern "C" fn rustls_result_is_cert_error (result : c_uint) -> bool { use rustls_result :: * ; matches ! (rustls_result :: from (result) , CertEncodingBad | CertExpired | CertNotYetValid | CertRevoked | CertUnhandledCriticalExtension | CertUnknownIssuer | CertUnknownRevocationStatus | CertBadSignature | CertNotValidForName | CertInvalidPurpose | CertApplicationVerificationFailure | CertOtherError) } }
};
}
