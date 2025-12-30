// Generated macro for set_read_secret (function)
macro_rules! Depcrate_tlsset_read_secret {
() => {
// Module: crate::tls
// Provides: {"set_read_secret"}
// Dependencies: {}
extern "C" fn set_read_secret (ssl : * mut SSL , level : crypto :: Level , cipher : * const SSL_CIPHER , secret : * const u8 , secret_len : usize ,) -> c_int { let ex_data = match ExData :: from_ssl_ptr (ssl) { Some (v) => v , None => return 0 , } ; trace ! ("{} set read secret lvl={:?}" , ex_data . trace_id , level) ; let space = match level { crypto :: Level :: Initial => & mut ex_data . crypto_ctx [packet :: Epoch :: Initial] , crypto :: Level :: ZeroRTT => & mut ex_data . crypto_ctx [packet :: Epoch :: Application] , crypto :: Level :: Handshake => & mut ex_data . crypto_ctx [packet :: Epoch :: Handshake] , crypto :: Level :: OneRTT => & mut ex_data . crypto_ctx [packet :: Epoch :: Application] , } ; let aead = match get_cipher_from_ptr (cipher) { Ok (v) => v , Err (_) => return 0 , } ; if level != crypto :: Level :: ZeroRTT || ex_data . is_server { let secret = unsafe { slice :: from_raw_parts (secret , secret_len) } ; let open = match crypto :: Open :: from_secret (aead , secret) { Ok (v) => v , Err (_) => return 0 , } ; if level == crypto :: Level :: ZeroRTT { space . crypto_0rtt_open = Some (open) ; return 1 ; } space . crypto_open = Some (open) ; } 1 }
};
}
