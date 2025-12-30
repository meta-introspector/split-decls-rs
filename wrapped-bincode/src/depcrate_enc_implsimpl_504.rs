// Generated macro for impl_504 (impl)
macro_rules! Depcrate_enc_implsimpl_504 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_504"}
// Dependencies: {}
impl Encode for bool { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { u8 :: from (* self) . encode (encoder) } }
};
}
