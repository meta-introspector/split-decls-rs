// Generated macro for impl_547 (impl)
macro_rules! Depcrate_enc_implsimpl_547 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_547"}
// Dependencies: {}
impl < T > Encode for Cell < T > where T : Encode + Copy , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (& self . get () , encoder) } }
};
}
