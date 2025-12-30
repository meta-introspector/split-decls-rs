// Generated macro for impl_654 (impl)
macro_rules! Depcrate_ops_wrappingimpl_654 {
() => {
// Module: crate::ops::wrapping
// Provides: {"impl_654"}
// Dependencies: {}
impl < T : WrappingAdd > WrappingAdd for Wrapping < T > where Wrapping < T > : Add < Output = Wrapping < T > > , { fn wrapping_add (& self , v : & Self) -> Self { Wrapping (self . 0 . wrapping_add (& v . 0)) } }
};
}
