// Generated macro for sk_X509_value (function)
macro_rules! Depcrate_tls_openssl_quictlssk_X509_value {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"sk_X509_value"}
// Dependencies: {}
# [allow (non_snake_case)] unsafe fn sk_X509_value (stack : * const STACK_OF , idx : usize) -> * mut c_void { OPENSSL_sk_value (stack as * const OPENSSL_STACK , idx) }
};
}
