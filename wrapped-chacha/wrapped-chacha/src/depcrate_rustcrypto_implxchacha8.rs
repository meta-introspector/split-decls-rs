// Generated macro for XChaCha8 (type)
macro_rules! Depcrate_rustcrypto_implXChaCha8 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"XChaCha8"}
// Dependencies: {}
# [doc = " Constructed analogously to XChaCha20, but with fewer rounds for higher performance;"] # [doc = " mixes during initialization to support both a long nonce and a full-length (64-bit) block counter."] pub type XChaCha8 = ChaChaAny < U24 , U4 , X > ;
};
}
