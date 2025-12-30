// Generated macro for rustls_certificate_get_der (function)
macro_rules! Depcrate_certificaterustls_certificate_get_der {
() => {
// Module: crate::certificate
// Provides: {"rustls_certificate_get_der"}
// Dependencies: {}
# [doc = " Get the DER data of the certificate itself."] # [doc = " The data is owned by the certificate and has the same lifetime."] # [no_mangle] pub extern "C" fn rustls_certificate_get_der (cert : * const rustls_certificate , out_der_data : * mut * const u8 , out_der_len : * mut size_t ,) -> rustls_result { ffi_panic_boundary ! { let cert = try_ref_from_ptr ! (cert) ; if out_der_data . is_null () || out_der_len . is_null () { return rustls_result :: NullParameter ; } let der = cert . as_ref () ; unsafe { * out_der_data = der . as_ptr () ; * out_der_len = der . len () ; } rustls_result :: Ok } }
};
}
