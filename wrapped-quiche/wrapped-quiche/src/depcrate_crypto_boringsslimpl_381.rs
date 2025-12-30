// Generated macro for impl_381 (impl)
macro_rules! Depcrate_crypto_boringsslimpl_381 {
() => {
// Module: crate::crypto::boringssl
// Provides: {"impl_381"}
// Dependencies: {}
impl Algorithm { fn get_evp_aead (self) -> * const EVP_AEAD { match self { Algorithm :: AES128_GCM => unsafe { EVP_aead_aes_128_gcm_tls13 () } , Algorithm :: AES256_GCM => unsafe { EVP_aead_aes_256_gcm_tls13 () } , Algorithm :: ChaCha20_Poly1305 => unsafe { EVP_aead_chacha20_poly1305 () } , } } }
};
}
