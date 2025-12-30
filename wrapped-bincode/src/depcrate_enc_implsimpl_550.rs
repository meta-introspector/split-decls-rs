// Generated macro for impl_550 (impl)
macro_rules! Depcrate_enc_implsimpl_550 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_550"}
// Dependencies: {}
impl < T > Encode for Range < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . start . encode (encoder) ? ; self . end . encode (encoder) ? ; Ok (()) } }
};
}
