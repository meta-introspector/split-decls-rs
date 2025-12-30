// Generated macro for impl_319 (impl)
macro_rules! Depcrate_extimpl_319 {
() => {
// Module: crate::ext
// Provides: {"impl_319"}
// Dependencies: {}
impl < T > Pointer for * mut T { unsafe fn distance (self , origin : * mut T) -> usize { (self as * const T) . distance (origin as * const T) } fn as_usize (self) -> usize { (self as * const T) . as_usize () } }
};
}
