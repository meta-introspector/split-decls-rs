// Generated macro for impl_471 (impl)
macro_rules! Depcrate_wrappingimpl_471 {
() => {
// Module: crate::wrapping
// Provides: {"impl_471"}
// Dependencies: {}
impl < T : WrappingAdd > Add < Self > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn add (self , rhs : Self) -> Self :: Output { Wrapping (self . 0 . wrapping_add (& rhs . 0)) } }
};
}
