// Generated macro for impl_572 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reimpl_572 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"impl_572"}
// Dependencies: {}
impl RingElementNTT { pub fn zero () -> Self { Self { coefficients : [FieldElement :: zero () ; 256] , } } # [doc = " NOTE: This should not be accessible by a user."] pub (crate) fn copy_from_non_ntt (not_ntt : & RingElement) -> Self { Self { coefficients : not_ntt . coefficients , } } # [doc = " FIPS-203, Algorithm 12."] pub fn base_case_multiply (a0 : FieldElement , a1 : FieldElement , b0 : FieldElement , b1 : FieldElement , gamma : FieldElement ,) -> (FieldElement , FieldElement) { let c0 : FieldElement = a0 * b0 + a1 * b1 * gamma ; let c1 : FieldElement = a0 * b1 + a1 * b0 ; (c0 , c1) } }
};
}
