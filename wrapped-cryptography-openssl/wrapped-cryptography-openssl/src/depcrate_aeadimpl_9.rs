// Generated macro for impl_9 (impl)
macro_rules! Depcrate_aeadimpl_9 {
() => {
// Module: crate::aead
// Provides: {"impl_9"}
// Dependencies: {}
impl AeadCtx { pub fn new (aead : AeadType , key : & [u8]) -> OpenSSLResult < AeadCtx > { let aead = match aead { AeadType :: ChaCha20Poly1305 => unsafe { ffi :: EVP_aead_chacha20_poly1305 () } , AeadType :: Aes128GcmSiv => unsafe { ffi :: EVP_aead_aes_128_gcm_siv () } , AeadType :: Aes256GcmSiv => unsafe { ffi :: EVP_aead_aes_256_gcm_siv () } , } ; let key_ptr = key . as_ptr () ; let tag_len = ffi :: EVP_AEAD_DEFAULT_TAG_LENGTH as usize ; unsafe { let ctx = cvt_p (ffi :: EVP_AEAD_CTX_new (aead , key_ptr , key . len () , tag_len)) ? ; Ok (AeadCtx :: from_ptr (ctx)) } } }
};
}
