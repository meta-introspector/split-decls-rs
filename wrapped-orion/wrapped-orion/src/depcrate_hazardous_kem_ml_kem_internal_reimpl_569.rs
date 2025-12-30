// Generated macro for impl_569 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reimpl_569 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"impl_569"}
// Dependencies: {}
impl Sub for RingElement { type Output = Self ; fn sub (self , other : Self) -> Self { let mut ret_sub = Self :: zero () ; sub_poly (& self . coefficients , & other . coefficients , & mut ret_sub . coefficients ,) ; ret_sub } }
};
}
