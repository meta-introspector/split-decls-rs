// Generated macro for impl_31 (impl)
macro_rules! Depcrate_aeadimpl_31 {
() => {
// Module: crate::aead
// Provides: {"impl_31"}
// Dependencies: {}
impl < N : NonceSequence > BoundKey < N > for OpeningKey < N > { fn new (key : UnboundKey , nonce_sequence : N) -> Self { Self { key , nonce_sequence , } } # [inline] fn algorithm (& self) -> & 'static Algorithm { self . key . algorithm () } }
};
}
