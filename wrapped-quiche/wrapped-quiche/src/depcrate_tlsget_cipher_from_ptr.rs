// Generated macro for get_cipher_from_ptr (function)
macro_rules! Depcrate_tlsget_cipher_from_ptr {
() => {
// Module: crate::tls
// Provides: {"get_cipher_from_ptr"}
// Dependencies: {}
fn get_cipher_from_ptr (cipher : * const SSL_CIPHER) -> Result < crypto :: Algorithm > { let cipher_id = unsafe { SSL_CIPHER_get_id (cipher) } ; let alg = match cipher_id { 0x0300_1301 => crypto :: Algorithm :: AES128_GCM , 0x0300_1302 => crypto :: Algorithm :: AES256_GCM , 0x0300_1303 => crypto :: Algorithm :: ChaCha20_Poly1305 , _ => return Err (Error :: TlsFail) , } ; Ok (alg) }
};
}
