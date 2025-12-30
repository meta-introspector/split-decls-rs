// Generated macro for impl_517 (impl)
macro_rules! Depcrate_enc_implsimpl_517 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_517"}
// Dependencies: {}
impl Encode for i8 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encoder . writer () . write (& [* self as u8]) } }
};
}
