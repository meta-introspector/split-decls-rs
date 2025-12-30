// Generated macro for impl_553 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_feimpl_553 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::fe
// Provides: {"impl_553"}
// Dependencies: {}
impl Mul for FieldElement { type Output = Self ; fn mul (self , other : Self) -> Self { Self (barrett_reduce (self . 0 * other . 0)) } }
};
}
