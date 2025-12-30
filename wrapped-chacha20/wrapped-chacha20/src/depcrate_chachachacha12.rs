// Generated macro for ChaCha12 (type)
macro_rules! Depcrate_chachaChaCha12 {
() => {
// Module: crate::chacha
// Provides: {"ChaCha12"}
// Dependencies: {}
# [doc = " ChaCha12 stream cipher (reduced-round variant of [`ChaCha20`] with 12 rounds)"] pub type ChaCha12 = StreamCipherCoreWrapper < ChaChaCore < R12 , Ietf > > ;
};
}
