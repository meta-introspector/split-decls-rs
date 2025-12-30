// Generated macro for impl_740 (impl)
macro_rules! Depcrate_rc_allocated_partial_initimpl_740 {
() => {
// Module: crate::rc::allocated_partial_init
// Provides: {"impl_740"}
// Dependencies: {}
impl < T : ? Sized > Drop for PartialInit < T > { # [inline] fn drop (& mut self) { unsafe { objc_release_fast (self . ptr as * mut _) } ; } }
};
}
