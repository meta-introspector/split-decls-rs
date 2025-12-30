// Generated macro for impl_522 (impl)
macro_rules! Depcrate_enc_implsimpl_522 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_522"}
// Dependencies: {}
impl Encode for NonZeroI32 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . get () . encode (encoder) } }
};
}
