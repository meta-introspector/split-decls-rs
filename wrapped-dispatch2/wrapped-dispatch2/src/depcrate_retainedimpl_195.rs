// Generated macro for impl_195 (impl)
macro_rules! Depcrate_retainedimpl_195 {
() => {
// Module: crate::retained
// Provides: {"impl_195"}
// Dependencies: {}
impl < T : ? Sized > Drop for DispatchRetained < T > { # [doc = " Releases the contained object."] # [doc (alias = "dispatch_release")] # [doc (alias = "release")] # [inline] fn drop (& mut self) { unsafe { dispatch_release (self . ptr . cast ()) } ; } }
};
}
