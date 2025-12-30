// Generated macro for impl_485 (impl)
macro_rules! Depcrate_wrappingimpl_485 {
() => {
// Module: crate::wrapping
// Provides: {"impl_485"}
// Dependencies: {}
impl < T : WrappingSub > Sub < Wrapping < T > > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn sub (self , rhs : Wrapping < T >) -> Self :: Output { Wrapping (self . 0 . wrapping_sub (& rhs . 0)) } }
};
}
