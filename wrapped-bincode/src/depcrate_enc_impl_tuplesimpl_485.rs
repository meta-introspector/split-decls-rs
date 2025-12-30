// Generated macro for impl_485 (impl)
macro_rules! Depcrate_enc_impl_tuplesimpl_485 {
() => {
// Module: crate::enc::impl_tuples
// Provides: {"impl_485"}
// Dependencies: {}
impl < A , B , C , D , E > Encode for (A , B , C , D , E) where A : Encode , B : Encode , C : Encode , D : Encode , E : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; self . 1 . encode (encoder) ? ; self . 2 . encode (encoder) ? ; self . 3 . encode (encoder) ? ; self . 4 . encode (encoder) ? ; Ok (()) } }
};
}
