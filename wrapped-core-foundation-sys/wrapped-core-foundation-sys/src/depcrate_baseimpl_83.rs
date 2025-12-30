// Generated macro for impl_83 (impl)
macro_rules! Depcrate_baseimpl_83 {
() => {
// Module: crate::base
// Provides: {"impl_83"}
// Dependencies: {}
impl < T > TCFTypeRef for * mut T { fn as_void_ptr (& self) -> * const c_void { (* self) as * const T as * const c_void } unsafe fn from_void_ptr (ptr : * const c_void) -> Self { ptr as * const T as * mut T } }
};
}
