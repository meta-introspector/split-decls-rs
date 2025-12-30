// Generated macro for impl_626 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internalimpl_626 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal
// Provides: {"impl_626"}
// Dependencies: {}
impl < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > Drop for DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > { fn drop (& mut self) { use zeroize :: Zeroize ; self . bytes . iter_mut () . zeroize () ; self . s_hat . iter_mut () . zeroize () ; } }
};
}
