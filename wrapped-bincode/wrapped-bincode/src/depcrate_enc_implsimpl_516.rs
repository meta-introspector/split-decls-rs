// Generated macro for impl_516 (impl)
macro_rules! Depcrate_enc_implsimpl_516 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_516"}
// Dependencies: {}
impl Encode for NonZeroUsize { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
