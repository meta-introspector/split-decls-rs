// Generated macro for impl_514 (impl)
macro_rules! Depcrate_enc_implsimpl_514 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_514"}
// Dependencies: {}
impl Encode for NonZeroU128 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
