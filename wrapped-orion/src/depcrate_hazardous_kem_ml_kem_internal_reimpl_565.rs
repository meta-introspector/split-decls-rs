// Generated macro for impl_565 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reimpl_565 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"impl_565"}
// Dependencies: {}
impl RingElement { pub fn zero () -> Self { Self { coefficients : [FieldElement :: zero () ; 256] , } } # [doc = " NOTE: This should not be accessible by a user."] pub (crate) fn copy_from_ntt (ntt : & RingElementNTT) -> Self { Self { coefficients : ntt . coefficients , } } # [cfg (all (test , feature = "safe_api"))] pub (crate) fn random_element () -> Self { use crate :: hazardous :: kem :: ml_kem :: internal :: fe :: KYBER_Q ; use rand :: { prelude :: * , rng } ; let mut rng = rng () ; let mut coefficients = [FieldElement :: zero () ; 256] ; for rand_coeff in coefficients . iter_mut () { let new = rng . random_range (0 .. KYBER_Q) ; * rand_coeff = FieldElement :: new (new) ; } Self { coefficients } } }
};
}
