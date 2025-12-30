// Generated macro for impl_398 (impl)
macro_rules! Depcrate_crypto_openssl_quictlsimpl_398 {
() => {
// Module: crate::crypto::openssl_quictls
// Provides: {"impl_398"}
// Dependencies: {}
impl Algorithm { pub fn get_evp (self) -> * const EVP_AEAD { match self { Algorithm :: AES128_GCM => unsafe { EVP_aes_128_ctr () } , Algorithm :: AES256_GCM => unsafe { EVP_aes_256_ctr () } , Algorithm :: ChaCha20_Poly1305 => unsafe { EVP_chacha20 () } , } } pub fn get_evp_aead (self) -> * const EVP_AEAD { match self { Algorithm :: AES128_GCM => unsafe { EVP_aes_128_gcm () } , Algorithm :: AES256_GCM => unsafe { EVP_aes_256_gcm () } , Algorithm :: ChaCha20_Poly1305 => unsafe { EVP_chacha20_poly1305 () } , } } }
};
}
