// Generated macro for impl_534 (impl)
macro_rules! Depcrate_enc_implsimpl_534 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_534"}
// Dependencies: {}
impl < T > Encode for [T] where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { super :: encode_slice_len (encoder , self . len ()) ? ; if unty :: type_equal :: < T , u8 > () { let t : & [u8] = unsafe { core :: mem :: transmute (self) } ; encoder . writer () . write (t) ? ; return Ok (()) ; } for item in self { item . encode (encoder) ? ; } Ok (()) } }
};
}
