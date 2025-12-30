// Generated macro for BoundKey (trait)
macro_rules! Depcrate_aeadBoundKey {
() => {
// Module: crate::aead
// Provides: {"BoundKey"}
// Dependencies: {}
# [doc = " An AEAD key bound to a nonce sequence."] pub trait BoundKey < N : NonceSequence > : core :: fmt :: Debug { # [doc = " Constructs a new key from the given `UnboundKey` and `NonceSequence`."] fn new (key : UnboundKey , nonce_sequence : N) -> Self ; # [doc = " The key's AEAD algorithm."] fn algorithm (& self) -> & 'static Algorithm ; }
};
}
