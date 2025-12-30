// Generated macro for impl_492 (impl)
macro_rules! Depcrate_wrappingimpl_492 {
() => {
// Module: crate::wrapping
// Provides: {"impl_492"}
// Dependencies: {}
impl < T : WrappingNeg > Neg for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn neg (self) -> Self :: Output { Wrapping (self . 0 . wrapping_neg ()) } }
};
}
