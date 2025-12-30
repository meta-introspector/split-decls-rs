// Generated macro for EvpCipherAead (struct)
macro_rules! Depcrate_backend_aeadEvpCipherAead {
() => {
// Module: crate::backend::aead
// Provides: {"EvpCipherAead"}
// Dependencies: {}
struct EvpCipherAead { base_encryption_ctx : openssl :: cipher_ctx :: CipherCtx , base_decryption_ctx : openssl :: cipher_ctx :: CipherCtx , tag_len : usize , tag_first : bool , }
};
}
