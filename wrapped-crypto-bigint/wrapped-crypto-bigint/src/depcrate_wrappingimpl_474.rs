// Generated macro for impl_474 (impl)
macro_rules! Depcrate_wrappingimpl_474 {
() => {
// Module: crate::wrapping
// Provides: {"impl_474"}
// Dependencies: {}
impl < T : WrappingAdd > Add < & Wrapping < T > > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn add (self , rhs : & Wrapping < T >) -> Self :: Output { Wrapping (self . 0 . wrapping_add (& rhs . 0)) } }
};
}
