// Generated macro for impl_568 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reimpl_568 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"impl_568"}
// Dependencies: {}
impl Add for RingElement { type Output = Self ; fn add (self , other : Self) -> Self { let mut ret_add = Self :: zero () ; add_poly (& self . coefficients , & other . coefficients , & mut ret_add . coefficients ,) ; ret_add } }
};
}
