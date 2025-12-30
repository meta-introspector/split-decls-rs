// Generated macro for impl_495 (impl)
macro_rules! Depcrate_wrappingimpl_495 {
() => {
// Module: crate::wrapping
// Provides: {"impl_495"}
// Dependencies: {}
impl < T : WrappingShr > Shr < u32 > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn shr (self , rhs : u32) -> Self :: Output { Wrapping (self . 0 . wrapping_shr (rhs)) } }
};
}
