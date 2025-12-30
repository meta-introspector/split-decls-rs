// Generated macro for impl_659 (impl)
macro_rules! Depcrate_ops_wrappingimpl_659 {
() => {
// Module: crate::ops::wrapping
// Provides: {"impl_659"}
// Dependencies: {}
impl < T : WrappingShr > WrappingShr for Wrapping < T > where Wrapping < T > : Shr < usize , Output = Wrapping < T > > , { fn wrapping_shr (& self , rhs : u32) -> Self { Wrapping (self . 0 . wrapping_shr (rhs)) } }
};
}
