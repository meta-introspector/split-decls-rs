// Generated macro for impl_551 (impl)
macro_rules! Depcrate_enc_implsimpl_551 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_551"}
// Dependencies: {}
impl < T > Encode for RangeInclusive < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . start () . encode (encoder) ? ; self . end () . encode (encoder) ? ; Ok (()) } }
};
}
