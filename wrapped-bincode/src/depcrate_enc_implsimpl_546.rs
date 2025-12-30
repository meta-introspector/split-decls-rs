// Generated macro for impl_546 (impl)
macro_rules! Depcrate_enc_implsimpl_546 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_546"}
// Dependencies: {}
impl < T , U > Encode for Result < T , U > where T : Encode , U : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self { Ok (val) => { 0u32 . encode (encoder) ? ; val . encode (encoder) } Err (err) => { 1u32 . encode (encoder) ? ; err . encode (encoder) } } } }
};
}
