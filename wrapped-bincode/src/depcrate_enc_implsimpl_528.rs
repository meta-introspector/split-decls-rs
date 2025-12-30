// Generated macro for impl_528 (impl)
macro_rules! Depcrate_enc_implsimpl_528 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_528"}
// Dependencies: {}
impl Encode for NonZeroIsize { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
