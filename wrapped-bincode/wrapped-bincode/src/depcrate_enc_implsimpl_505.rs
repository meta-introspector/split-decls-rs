// Generated macro for impl_505 (impl)
macro_rules! Depcrate_enc_implsimpl_505 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_505"}
// Dependencies: {}
impl Encode for u8 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encoder . writer () . write (& [* self]) } }
};
}
