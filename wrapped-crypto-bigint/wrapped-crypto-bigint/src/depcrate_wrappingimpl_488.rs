// Generated macro for impl_488 (impl)
macro_rules! Depcrate_wrappingimpl_488 {
() => {
// Module: crate::wrapping
// Provides: {"impl_488"}
// Dependencies: {}
impl < T : WrappingMul > Mul < & Self > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn mul (self , rhs : & Self) -> Self :: Output { Wrapping (self . 0 . wrapping_mul (& rhs . 0)) } }
};
}
