// Generated macro for impl_486 (impl)
macro_rules! Depcrate_enc_impl_tuplesimpl_486 {
() => {
// Module: crate::enc::impl_tuples
// Provides: {"impl_486"}
// Dependencies: {}
impl < A , B , C , D , E , F > Encode for (A , B , C , D , E , F) where A : Encode , B : Encode , C : Encode , D : Encode , E : Encode , F : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; self . 4 . encode (encoder) ? ; self . 5 . encode (encoder) ? ; Ok (()) } }
};
}
