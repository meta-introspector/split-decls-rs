// Generated macro for impl_35 (impl)
macro_rules! Depcrate_aeadimpl_35 {
() => {
// Module: crate::aead
// Provides: {"impl_35"}
// Dependencies: {}
impl < N : NonceSequence > BoundKey < N > for SealingKey < N > { fn new (key : UnboundKey , nonce_sequence : N) -> Self { Self { key , nonce_sequence , } } # [inline] fn algorithm (& self) -> & 'static Algorithm { self . key . algorithm () } }
};
}
