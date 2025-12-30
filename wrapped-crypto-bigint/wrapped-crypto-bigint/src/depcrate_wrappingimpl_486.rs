// Generated macro for impl_486 (impl)
macro_rules! Depcrate_wrappingimpl_486 {
() => {
// Module: crate::wrapping
// Provides: {"impl_486"}
// Dependencies: {}
impl < T : WrappingShl > Shl < u32 > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn shl (self , rhs : u32) -> Self :: Output { Wrapping (self . 0 . wrapping_shl (rhs)) } }
};
}
