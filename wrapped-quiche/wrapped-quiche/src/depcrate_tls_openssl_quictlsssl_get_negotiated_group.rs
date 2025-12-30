// Generated macro for SSL_get_negotiated_group (function)
macro_rules! Depcrate_tls_openssl_quictlsSSL_get_negotiated_group {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"SSL_get_negotiated_group"}
// Dependencies: {}
# [allow (non_snake_case)] unsafe fn SSL_get_negotiated_group (ssl : * const SSL) -> c_int { const SSL_CTRL_GET_NEGOTIATED_GROUP : c_int = 134 ; SSL_ctrl (ssl , SSL_CTRL_GET_NEGOTIATED_GROUP , 0 as c_long , ptr :: null_mut () ,) as c_int }
};
}
