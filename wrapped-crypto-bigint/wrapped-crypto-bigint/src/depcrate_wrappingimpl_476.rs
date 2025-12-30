// Generated macro for impl_476 (impl)
macro_rules! Depcrate_wrappingimpl_476 {
() => {
// Module: crate::wrapping
// Provides: {"impl_476"}
// Dependencies: {}
impl < T : WrappingSub > Sub < & Self > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn sub (self , rhs : & Self) -> Self :: Output { Wrapping (self . 0 . wrapping_sub (& rhs . 0)) } }
};
}
