// Generated macro for SSL_QUIC_METHOD (struct)
macro_rules! Depcrate_tls_openssl_quictlsSSL_QUIC_METHOD {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"SSL_QUIC_METHOD"}
// Dependencies: {}
# [repr (C)] # [allow (non_camel_case_types)] pub (super) struct SSL_QUIC_METHOD { set_encryption_secrets : Option < extern "C" fn (ssl : * mut SSL , level : crypto :: Level , read_secret : * const u8 , write_secret : * const u8 , secret_len : usize ,) -> c_int , > , add_handshake_data : Option < unsafe extern "C" fn (ssl : * mut SSL , level : crypto :: Level , data : * const u8 , len : usize ,) -> c_int , > , flush_flight : Option < extern "C" fn (ssl : * mut SSL) -> c_int > , send_alert : Option < extern "C" fn (ssl : * mut SSL , level : crypto :: Level , alert : u8) -> c_int , > , }
};
}
