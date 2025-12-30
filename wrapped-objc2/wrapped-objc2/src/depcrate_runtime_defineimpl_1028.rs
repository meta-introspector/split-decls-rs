// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_runtime_defineimpl_1028 {
() => {
// Module: crate::runtime::define
// Provides: {"impl_1028"}
// Dependencies: {}
impl Drop for ClassBuilder { # [inline] fn drop (& mut self) { # [cfg (feature = "gnustep-1-7")] unsafe { ffi :: objc_registerClassPair (self . as_mut_ptr ()) ; } unsafe { ffi :: objc_disposeClassPair (self . as_mut_ptr ()) } } }
};
}
