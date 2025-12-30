// Generated macro for impl_518 (impl)
macro_rules! Depcrate_enc_implsimpl_518 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_518"}
// Dependencies: {}
impl Encode for NonZeroI8 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
