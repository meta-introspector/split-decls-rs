// Generated macro for impl_481 (impl)
macro_rules! Depcrate_enc_impl_tuplesimpl_481 {
() => {
// Module: crate::enc::impl_tuples
// Provides: {"impl_481"}
// Dependencies: {}
impl < A > Encode for (A ,) where A : Encode , { fn encode < _E : Encoder > (& self , encoder : & mut _E) -> Result < () , EncodeError > { self . 0 . encode (encoder) ? ; Ok (()) } }
};
}
