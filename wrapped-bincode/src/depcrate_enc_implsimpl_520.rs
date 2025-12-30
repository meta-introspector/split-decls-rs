// Generated macro for impl_520 (impl)
macro_rules! Depcrate_enc_implsimpl_520 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_520"}
// Dependencies: {}
impl Encode for NonZeroI16 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
