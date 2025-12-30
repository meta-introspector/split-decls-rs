// Generated macro for impl_490 (impl)
macro_rules! Depcrate_enc_impl_tuplesimpl_490 {
() => {
// Module: crate::enc::impl_tuples
// Provides: {"impl_490"}
// Dependencies: {}
impl < A , B , C , D , E , F , G , H , I , J > Encode for (A , B , C , D , E , F , G , H , I , J) where A : Encode , B : Encode , C : Encode , D : Encode , E : Encode , F : Encode , G : Encode , H : Encode , I : Encode , J : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; self . 4 . encode (encoder) ? ; self . 5 . encode (encoder) ? ; self . 6 . encode (encoder) ? ; self . 7 . encode (encoder) ? ; self . 8 . encode (encoder) ? ; self . 9 . encode (encoder) ? ; Ok (()) } }
};
}
