// Generated macro for impl_531 (impl)
macro_rules! Depcrate_enc_implsimpl_531 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_531"}
// Dependencies: {}
impl < T : Encode > Encode for Wrapping < T > { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . 0 . encode (encoder) } }
};
}
