// Generated macro for impl_553 (impl)
macro_rules! Depcrate_enc_implsimpl_553 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_553"}
// Dependencies: {}
impl < T > Encode for & T where T : Encode + ? Sized , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { T :: encode (self , encoder) } }
};
}
