// Generated macro for impl_82 (impl)
macro_rules! Depcrate_baseimpl_82 {
() => {
// Module: crate::base
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > TCFTypeRef for * const T { fn as_void_ptr (& self) -> * const c_void { (* self) as * const c_void } unsafe fn from_void_ptr (ptr : * const c_void) -> Self { ptr as * const T } }
};
}
