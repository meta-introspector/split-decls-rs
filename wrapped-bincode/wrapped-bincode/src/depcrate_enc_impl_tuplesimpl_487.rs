// Generated macro for impl_487 (impl)
macro_rules! Depcrate_enc_impl_tuplesimpl_487 {
() => {
// Module: crate::enc::impl_tuples
// Provides: {"impl_487"}
// Dependencies: {}
impl < A , B , C , D , E , F , G > Encode for (A , B , C , D , E , F , G) where A : Encode , B : Encode , C : Encode , D : Encode , E : Encode , F : Encode , G : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; self . 4 . encode (encoder) ? ; self . 5 . encode (encoder) ? ; self . 6 . encode (encoder) ? ; Ok (()) } }
};
}
