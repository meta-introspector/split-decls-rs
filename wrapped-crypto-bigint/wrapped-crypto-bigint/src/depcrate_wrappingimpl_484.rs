// Generated macro for impl_484 (impl)
macro_rules! Depcrate_wrappingimpl_484 {
() => {
// Module: crate::wrapping
// Provides: {"impl_484"}
// Dependencies: {}
impl < T : WrappingNeg > Neg for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn neg (self) -> Self :: Output { Wrapping (self . 0 . wrapping_neg ()) } }
};
}
