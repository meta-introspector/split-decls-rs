// Generated macro for SealingKey (struct)
macro_rules! Depcrate_aeadSealingKey {
() => {
// Module: crate::aead
// Provides: {"SealingKey"}
// Dependencies: {}
# [doc = " An AEAD key for encrypting and signing (\"sealing\"), bound to a nonce"] # [doc = " sequence."] # [doc = ""] # [doc = " Intentionally not `Clone` or `Copy` since cloning would allow duplication"] # [doc = " of the nonce sequence."] # [doc = ""] # [doc = " Prefer [`RandomizedNonceKey`] for sealing operations."] pub struct SealingKey < N : NonceSequence > { key : UnboundKey , nonce_sequence : N , }
};
}
