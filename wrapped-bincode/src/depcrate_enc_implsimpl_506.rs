// Generated macro for impl_506 (impl)
macro_rules! Depcrate_enc_implsimpl_506 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_506"}
// Dependencies: {}
impl Encode for NonZeroU8 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
