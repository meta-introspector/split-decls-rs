// Generated macro for impl_551 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_feimpl_551 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::fe
// Provides: {"impl_551"}
// Dependencies: {}
impl Add for FieldElement { type Output = Self ; fn add (self , other : Self) -> Self { let x : u32 = self . 0 + other . 0 ; Self (conditional_sub_u32 (x)) } }
};
}
