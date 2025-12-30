// Generated macro for impl_488 (impl)
macro_rules! Depcrate_wrappingimpl_488 {
() => {
// Module: crate::wrapping
// Provides: {"impl_488"}
// Dependencies: {}
impl < T : WrappingShr > Shr < u32 > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn shr (self , rhs : u32) -> Self :: Output { Wrapping (self . 0 . wrapping_shr (rhs)) } }
};
}
