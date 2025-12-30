// Generated macro for impl_483 (impl)
macro_rules! Depcrate_wrappingimpl_483 {
() => {
// Module: crate::wrapping
// Provides: {"impl_483"}
// Dependencies: {}
impl < T : WrappingSub > Sub < Self > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn sub (self , rhs : Self) -> Self :: Output { Wrapping (self . 0 . wrapping_sub (& rhs . 0)) } }
};
}
