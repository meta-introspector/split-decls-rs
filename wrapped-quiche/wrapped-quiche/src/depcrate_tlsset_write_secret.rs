// Generated macro for set_write_secret (function)
macro_rules! Depcrate_tlsset_write_secret {
() => {
// Module: crate::tls
// Provides: {"set_write_secret"}
// Dependencies: {}
extern "C" fn set_write_secret (ssl : * mut SSL , level : crypto :: Level , cipher : * const SSL_CIPHER , secret : * const u8 , secret_len : usize ,) -> c_int { let ex_data = match ExData :: from_ssl_ptr (ssl) { Some (v) => v , None => return 0 , } ; trace ! ("{} set write secret lvl={:?}" , ex_data . trace_id , level) ; let space = match level { crypto :: Level :: Initial => & mut ex_data . crypto_ctx [packet :: Epoch :: Initial] , crypto :: Level :: ZeroRTT => & mut ex_data . crypto_ctx [packet :: Epoch :: Application] , crypto :: Level :: Handshake => & mut ex_data . crypto_ctx [packet :: Epoch :: Handshake] , crypto :: Level :: OneRTT => & mut ex_data . crypto_ctx [packet :: Epoch :: Application] , } ; let aead = match get_cipher_from_ptr (cipher) { Ok (v) => v , Err (_) => return 0 , } ; if level != crypto :: Level :: ZeroRTT || ! ex_data . is_server { let secret = unsafe { slice :: from_raw_parts (secret , secret_len) } ; let seal = match crypto :: Seal :: from_secret (aead , secret) { Ok (v) => v , Err (_) => return 0 , } ; space . crypto_seal = Some (seal) ; } 1 }
};
}
