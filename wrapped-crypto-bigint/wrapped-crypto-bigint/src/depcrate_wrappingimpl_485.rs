// Generated macro for impl_485 (impl)
macro_rules! Depcrate_wrappingimpl_485 {
() => {
// Module: crate::wrapping
// Provides: {"impl_485"}
// Dependencies: {}
impl < T : WrappingShl > Shl < u32 > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn shl (self , rhs : u32) -> Self :: Output { Wrapping (self . 0 . wrapping_shl (rhs)) } }
};
}
