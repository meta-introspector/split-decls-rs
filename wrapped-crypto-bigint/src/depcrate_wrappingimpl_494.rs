// Generated macro for impl_494 (impl)
macro_rules! Depcrate_wrappingimpl_494 {
() => {
// Module: crate::wrapping
// Provides: {"impl_494"}
// Dependencies: {}
impl < T : WrappingShl > Shl < u32 > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn shl (self , rhs : u32) -> Self :: Output { Wrapping (self . 0 . wrapping_shl (rhs)) } }
};
}
