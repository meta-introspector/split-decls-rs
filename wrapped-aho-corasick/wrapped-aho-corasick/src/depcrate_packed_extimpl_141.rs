// Generated macro for impl_141 (impl)
macro_rules! Depcrate_packed_extimpl_141 {
() => {
// Module: crate::packed::ext
// Provides: {"impl_141"}
// Dependencies: {}
impl < T > Pointer for * mut T { unsafe fn distance (self , origin : * mut T) -> usize { (self as * const T) . distance (origin as * const T) } fn as_usize (self) -> usize { (self as * const T) . as_usize () } }
};
}
