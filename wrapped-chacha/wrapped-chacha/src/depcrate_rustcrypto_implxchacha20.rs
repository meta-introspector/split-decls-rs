// Generated macro for XChaCha20 (type)
macro_rules! Depcrate_rustcrypto_implXChaCha20 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"XChaCha20"}
// Dependencies: {}
# [doc = " Constructed analogously to XSalsa20; mixes during initialization to support both a long nonce"] # [doc = " and a full-length (64-bit) block counter."] pub type XChaCha20 = ChaChaAny < U24 , U10 , X > ;
};
}
