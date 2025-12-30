// Generated macro for impl_656 (impl)
macro_rules! Depcrate_ops_wrappingimpl_656 {
() => {
// Module: crate::ops::wrapping
// Provides: {"impl_656"}
// Dependencies: {}
impl < T : WrappingMul > WrappingMul for Wrapping < T > where Wrapping < T > : Mul < Output = Wrapping < T > > , { fn wrapping_mul (& self , v : & Self) -> Self { Wrapping (self . 0 . wrapping_mul (& v . 0)) } }
};
}
