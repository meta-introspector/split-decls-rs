// Generated macro for impl_483 (impl)
macro_rules! Depcrate_enc_impl_tuplesimpl_483 {
() => {
// Module: crate::enc::impl_tuples
// Provides: {"impl_483"}
// Dependencies: {}
impl < A , B , C > Encode for (A , B , C) where A : Encode , B : Encode , C : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; Ok (()) } }
};
}
