// Generated macro for LessSafeKey (struct)
macro_rules! Depcrate_aeadLessSafeKey {
() => {
// Module: crate::aead
// Provides: {"LessSafeKey"}
// Dependencies: {}
# [doc = " Immutable keys for use in situations where `OpeningKey`/`SealingKey` and"] # [doc = " `NonceSequence` cannot reasonably be used."] # [doc = ""] # [doc = " Prefer [`RandomizedNonceKey`] when practical."] pub struct LessSafeKey { key : UnboundKey , }
};
}
