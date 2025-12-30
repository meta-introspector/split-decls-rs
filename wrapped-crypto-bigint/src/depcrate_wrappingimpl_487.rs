// Generated macro for impl_487 (impl)
macro_rules! Depcrate_wrappingimpl_487 {
() => {
// Module: crate::wrapping
// Provides: {"impl_487"}
// Dependencies: {}
impl < T : WrappingMul > Mul < Self > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn mul (self , rhs : Self) -> Self :: Output { Wrapping (self . 0 . wrapping_mul (& rhs . 0)) } }
};
}
