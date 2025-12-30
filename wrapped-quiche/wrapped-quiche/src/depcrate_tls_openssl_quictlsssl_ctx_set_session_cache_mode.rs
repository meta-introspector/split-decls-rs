// Generated macro for SSL_CTX_set_session_cache_mode (function)
macro_rules! Depcrate_tls_openssl_quictlsSSL_CTX_set_session_cache_mode {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"SSL_CTX_set_session_cache_mode"}
// Dependencies: {}
# [allow (non_snake_case)] pub (super) unsafe fn SSL_CTX_set_session_cache_mode (ctx : * mut SSL_CTX , mode : c_int ,) -> c_int { const SSL_CTRL_SET_SESS_CACHE_MODE : c_int = 44 ; SSL_CTX_ctrl (ctx , SSL_CTRL_SET_SESS_CACHE_MODE , mode as c_long , ptr :: null_mut () ,) as c_int }
};
}
