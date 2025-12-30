// Generated macro for set_encryption_secrets (function)
macro_rules! Depcrate_tls_openssl_quictlsset_encryption_secrets {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"set_encryption_secrets"}
// Dependencies: {}
extern "C" fn set_encryption_secrets (ssl : * mut SSL , level : crypto :: Level , read_secret : * const u8 , write_secret : * const u8 , secret_len : usize ,) -> c_int { let cipher = map_result_ptr (unsafe { SSL_get_current_cipher (ssl) }) ; let _write_ret = set_write_secret (ssl , level , cipher . unwrap () , write_secret , secret_len) ; let _read_ret = set_read_secret (ssl , level , cipher . unwrap () , read_secret , secret_len) ; 1 }
};
}
