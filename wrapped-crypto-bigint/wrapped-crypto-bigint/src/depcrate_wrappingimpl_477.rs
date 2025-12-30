// Generated macro for impl_477 (impl)
macro_rules! Depcrate_wrappingimpl_477 {
() => {
// Module: crate::wrapping
// Provides: {"impl_477"}
// Dependencies: {}
impl < T : WrappingSub > Sub < Wrapping < T > > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn sub (self , rhs : Wrapping < T >) -> Self :: Output { Wrapping (self . 0 . wrapping_sub (& rhs . 0)) } }
};
}
