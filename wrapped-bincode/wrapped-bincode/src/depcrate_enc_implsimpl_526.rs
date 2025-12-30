// Generated macro for impl_526 (impl)
macro_rules! Depcrate_enc_implsimpl_526 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_526"}
// Dependencies: {}
impl Encode for NonZeroI128 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
