// Generated macro for impl_487 (impl)
macro_rules! Depcrate_wrappingimpl_487 {
() => {
// Module: crate::wrapping
// Provides: {"impl_487"}
// Dependencies: {}
impl < T : WrappingShr > Shr < u32 > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn shr (self , rhs : u32) -> Self :: Output { Wrapping (self . 0 . wrapping_shr (rhs)) } }
};
}
