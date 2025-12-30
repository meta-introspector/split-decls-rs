// Generated macro for impl_544 (impl)
macro_rules! Depcrate_enc_implsimpl_544 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_544"}
// Dependencies: {}
impl < T , const N : usize > Encode for [T ; N] where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { if unty :: type_equal :: < T , u8 > () { let array_slice : & [u8] = unsafe { core :: slice :: from_raw_parts (self . as_ptr () . cast () , N) } ; encoder . writer () . write (array_slice) } else { for item in self . iter () { item . encode (encoder) ? ; } Ok (()) } } }
};
}
