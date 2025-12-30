// Generated macro for impl_491 (impl)
macro_rules! Depcrate_wrappingimpl_491 {
() => {
// Module: crate::wrapping
// Provides: {"impl_491"}
// Dependencies: {}
impl < T : WrappingNeg > Neg for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn neg (self) -> Self :: Output { Wrapping (self . 0 . wrapping_neg ()) } }
};
}
