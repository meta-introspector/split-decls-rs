// Generated macro for OpeningKey (struct)
macro_rules! Depcrate_aeadOpeningKey {
() => {
// Module: crate::aead
// Provides: {"OpeningKey"}
// Dependencies: {}
# [doc = " An AEAD key for authenticating and decrypting (\"opening\"), bound to a nonce"] # [doc = " sequence."] # [doc = ""] # [doc = " Intentionally not `Clone` or `Copy` since cloning would allow duplication"] # [doc = " of the nonce sequence."] # [doc = ""] # [doc = " Prefer [`RandomizedNonceKey`] for opening operations."] pub struct OpeningKey < N : NonceSequence > { key : UnboundKey , nonce_sequence : N , }
};
}
