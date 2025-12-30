// Generated macro for impl_655 (impl)
macro_rules! Depcrate_ops_wrappingimpl_655 {
() => {
// Module: crate::ops::wrapping
// Provides: {"impl_655"}
// Dependencies: {}
impl < T : WrappingSub > WrappingSub for Wrapping < T > where Wrapping < T > : Sub < Output = Wrapping < T > > , { fn wrapping_sub (& self , v : & Self) -> Self { Wrapping (self . 0 . wrapping_sub (& v . 0)) } }
};
}
