// Generated macro for impl_552 (impl)
macro_rules! Depcrate_enc_implsimpl_552 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_552"}
// Dependencies: {}
impl < T > Encode for Bound < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self { Self :: Unbounded => { 0u32 . encode (encoder) ? ; } Self :: Included (val) => { 1u32 . encode (encoder) ? ; val . encode (encoder) ? ; } Self :: Excluded (val) => { 2u32 . encode (encoder) ? ; val . encode (encoder) ? ; } } Ok (()) } }
};
}
