// Generated macro for impl_736 (impl)
macro_rules! Depcrate_rc_allocated_partial_initimpl_736 {
() => {
// Module: crate::rc::allocated_partial_init
// Provides: {"impl_736"}
// Dependencies: {}
impl < T : ? Sized > Drop for Allocated < T > { # [inline] fn drop (& mut self) { unsafe { objc_release_fast (self . ptr as * mut _) } ; } }
};
}
