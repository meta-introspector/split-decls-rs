// Generated macro for SSL_get_ex_new_index (function)
macro_rules! Depcrate_tls_openssl_quictlsSSL_get_ex_new_index {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"SSL_get_ex_new_index"}
// Dependencies: {}
# [allow (non_snake_case)] pub (super) unsafe fn SSL_get_ex_new_index (argl : c_long , argp : * const c_void , newf : * const c_void , dupf : * const c_void , freef : * const c_void ,) -> c_int { const CRYPTO_EX_INDEX_SSL : c_int = 0 ; CRYPTO_get_ex_new_index (CRYPTO_EX_INDEX_SSL , argl , argp , newf , dupf , freef) }
};
}
