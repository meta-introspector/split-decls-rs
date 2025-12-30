// Generated macro for impl_545 (impl)
macro_rules! Depcrate_enc_implsimpl_545 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_545"}
// Dependencies: {}
impl < T > Encode for Option < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { super :: encode_option_variant (encoder , self) ? ; if let Some (val) = self { val . encode (encoder) ? ; } Ok (()) } }
};
}
