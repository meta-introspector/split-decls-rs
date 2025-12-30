// Generated macro for SSL_set_min_proto_version (function)
macro_rules! Depcrate_tls_openssl_quictlsSSL_set_min_proto_version {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"SSL_set_min_proto_version"}
// Dependencies: {}
# [allow (non_snake_case)] pub (super) unsafe fn SSL_set_min_proto_version (s : * mut SSL , version : u16 ,) -> c_int { const SSL_CTRL_SET_MIN_PROTO_VERSION : c_int = 123 ; SSL_ctrl (s , SSL_CTRL_SET_MIN_PROTO_VERSION , version as c_long , ptr :: null_mut () ,) as c_int }
};
}
