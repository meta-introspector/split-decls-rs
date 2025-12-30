// Generated macro for impl_486 (impl)
macro_rules! Depcrate_wrappingimpl_486 {
() => {
// Module: crate::wrapping
// Provides: {"impl_486"}
// Dependencies: {}
impl < T : WrappingSub > Sub < & Wrapping < T > > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn sub (self , rhs : & Wrapping < T >) -> Self :: Output { Wrapping (self . 0 . wrapping_sub (& rhs . 0)) } }
};
}
