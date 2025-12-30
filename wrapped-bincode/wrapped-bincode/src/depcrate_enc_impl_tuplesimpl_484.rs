// Generated macro for impl_484 (impl)
macro_rules! Depcrate_enc_impl_tuplesimpl_484 {
() => {
// Module: crate::enc::impl_tuples
// Provides: {"impl_484"}
// Dependencies: {}
impl < A , B , C , D > Encode for (A , B , C , D) where A : Encode , B : Encode , C : Encode , D : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; Ok (()) } }
};
}
