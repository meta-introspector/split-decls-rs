// Generated macro for impl_82 (impl)
macro_rules! Depcrate_ppcimpl_82 {
() => {
// Module: crate::ppc
// Provides: {"impl_82"}
// Dependencies: {}
impl < F : Float > Neg for DoubleFloat < F > { type Output = Self ; fn neg (self) -> Self { if self . 1 . is_finite_non_zero () { DoubleFloat (- self . 0 , - self . 1) } else { DoubleFloat (- self . 0 , self . 1) } } }
};
}
