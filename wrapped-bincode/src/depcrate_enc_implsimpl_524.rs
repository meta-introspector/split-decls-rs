// Generated macro for impl_524 (impl)
macro_rules! Depcrate_enc_implsimpl_524 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_524"}
// Dependencies: {}
impl Encode for NonZeroI64 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
