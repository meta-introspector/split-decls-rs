// Generated macro for impl_543 (impl)
macro_rules! Depcrate_enc_implsimpl_543 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_543"}
// Dependencies: {}
impl Encode for str { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_bytes () . encode (encoder) } }
};
}
