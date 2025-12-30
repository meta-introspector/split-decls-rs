// Generated macro for impl_552 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_feimpl_552 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::fe
// Provides: {"impl_552"}
// Dependencies: {}
impl Sub for FieldElement { type Output = Self ; fn sub (self , other : Self) -> Self { let x : u32 = self . 0 . overflowing_sub (other . 0) . 0 . overflowing_add (KYBER_Q) . 0 ; Self (conditional_sub_u32 (x)) } }
};
}
