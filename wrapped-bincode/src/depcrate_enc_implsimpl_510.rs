// Generated macro for impl_510 (impl)
macro_rules! Depcrate_enc_implsimpl_510 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_510"}
// Dependencies: {}
impl Encode for NonZeroU32 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
