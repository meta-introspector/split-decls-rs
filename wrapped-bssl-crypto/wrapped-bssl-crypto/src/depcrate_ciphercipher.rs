// Generated macro for Cipher (struct)
macro_rules! Depcrate_cipherCipher {
() => {
// Module: crate::cipher
// Provides: {"Cipher"}
// Dependencies: {}
# [doc = " Internal cipher implementation which wraps `EVP_CIPHER_*`"] struct Cipher < C : EvpCipherType > { ctx : * mut bssl_sys :: EVP_CIPHER_CTX , _marker : PhantomData < C > , }
};
}
