// Generated macro for SSL_CTX_set_tlsext_ticket_keys (function)
macro_rules! Depcrate_tls_openssl_quictlsSSL_CTX_set_tlsext_ticket_keys {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"SSL_CTX_set_tlsext_ticket_keys"}
// Dependencies: {}
# [allow (non_snake_case)] pub (super) unsafe fn SSL_CTX_set_tlsext_ticket_keys (ctx : * mut SSL_CTX , key : * const u8 , key_len : usize ,) -> c_int { const SSL_CTRL_SET_TLSEXT_TICKET_KEYS : c_int = 59 ; SSL_CTX_ctrl (ctx , SSL_CTRL_SET_TLSEXT_TICKET_KEYS , key_len as c_long , key as * mut c_void ,) as c_int }
};
}
