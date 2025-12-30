// Generated macro for impl_512 (impl)
macro_rules! Depcrate_enc_implsimpl_512 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_512"}
// Dependencies: {}
impl Encode for NonZeroU64 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
