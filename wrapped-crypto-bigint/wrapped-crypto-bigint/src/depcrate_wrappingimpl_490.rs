// Generated macro for impl_490 (impl)
macro_rules! Depcrate_wrappingimpl_490 {
() => {
// Module: crate::wrapping
// Provides: {"impl_490"}
// Dependencies: {}
impl < T : WrappingMul > Mul < & Wrapping < T > > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn mul (self , rhs : & Wrapping < T >) -> Self :: Output { Wrapping (self . 0 . wrapping_mul (& rhs . 0)) } }
};
}
