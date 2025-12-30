// Generated macro for impl_657 (impl)
macro_rules! Depcrate_ops_wrappingimpl_657 {
() => {
// Module: crate::ops::wrapping
// Provides: {"impl_657"}
// Dependencies: {}
impl < T : WrappingNeg > WrappingNeg for Wrapping < T > where Wrapping < T > : Neg < Output = Wrapping < T > > , { fn wrapping_neg (& self) -> Self { Wrapping (self . 0 . wrapping_neg ()) } }
};
}
