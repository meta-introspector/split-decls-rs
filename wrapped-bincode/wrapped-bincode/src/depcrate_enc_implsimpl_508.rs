// Generated macro for impl_508 (impl)
macro_rules! Depcrate_enc_implsimpl_508 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_508"}
// Dependencies: {}
impl Encode for NonZeroU16 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
