// Generated macro for impl_658 (impl)
macro_rules! Depcrate_ops_wrappingimpl_658 {
() => {
// Module: crate::ops::wrapping
// Provides: {"impl_658"}
// Dependencies: {}
impl < T : WrappingShl > WrappingShl for Wrapping < T > where Wrapping < T > : Shl < usize , Output = Wrapping < T > > , { fn wrapping_shl (& self , rhs : u32) -> Self { Wrapping (self . 0 . wrapping_shl (rhs)) } }
};
}
