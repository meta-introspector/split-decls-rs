// Generated macro for impl_482 (impl)
macro_rules! Depcrate_enc_impl_tuplesimpl_482 {
() => {
// Module: crate::enc::impl_tuples
// Provides: {"impl_482"}
// Dependencies: {}
impl < A , B > Encode for (A , B) where A : Encode , B : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; Ok (()) } }
};
}
