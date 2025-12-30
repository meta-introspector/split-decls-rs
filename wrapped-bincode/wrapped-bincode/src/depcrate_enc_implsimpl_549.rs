// Generated macro for impl_549 (impl)
macro_rules! Depcrate_enc_implsimpl_549 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_549"}
// Dependencies: {}
impl Encode for Duration { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { self . as_secs () . encode (encoder) ? ; self . subsec_nanos () . encode (encoder) ? ; Ok (()) } }
};
}
