// Generated macro for SSL_set_tlsext_host_name (function)
macro_rules! Depcrate_tls_openssl_quictlsSSL_set_tlsext_host_name {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"SSL_set_tlsext_host_name"}
// Dependencies: {}
# [allow (non_snake_case)] pub (super) unsafe fn SSL_set_tlsext_host_name (s : * mut SSL , name : * const c_char ,) -> c_int { const SSL_CTRL_SET_TLSEXT_HOSTNAME : c_int = 55 ; # [allow (non_upper_case_globals)] const TLSEXT_NAMETYPE_host_name : c_long = 0 ; SSL_ctrl (s , SSL_CTRL_SET_TLSEXT_HOSTNAME , TLSEXT_NAMETYPE_host_name , name as * mut c_void ,) as c_int }
};
}
