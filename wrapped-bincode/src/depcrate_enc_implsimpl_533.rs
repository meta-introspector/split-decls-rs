// Generated macro for impl_533 (impl)
macro_rules! Depcrate_enc_implsimpl_533 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_533"}
// Dependencies: {}
impl Encode for char { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { encode_utf8 (encoder . writer () , * self) } }
};
}
