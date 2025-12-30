// Generated macro for sk_X509_num (function)
macro_rules! Depcrate_tls_openssl_quictlssk_X509_num {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"sk_X509_num"}
// Dependencies: {}
# [allow (non_snake_case)] unsafe fn sk_X509_num (stack : * const STACK_OF) -> usize { OPENSSL_sk_num (stack as * const OPENSSL_STACK) }
};
}
