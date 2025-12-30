// Generated macro for impl_532 (impl)
macro_rules! Depcrate_enc_implsimpl_532 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_532"}
// Dependencies: {}
impl < T : Encode > Encode for Reverse < T > { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . 0 . encode (encoder) } }
};
}
