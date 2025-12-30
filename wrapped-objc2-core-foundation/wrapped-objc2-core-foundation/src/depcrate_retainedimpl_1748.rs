// Generated macro for impl_1748 (impl)
macro_rules! Depcrate_retainedimpl_1748 {
() => {
// Module: crate::retained
// Provides: {"impl_1748"}
// Dependencies: {}
impl < T : ? Sized > Drop for CFRetained < T > { # [doc = " Releases the contained type."] # [doc (alias = "CFRelease")] # [doc (alias = "release")] # [inline] fn drop (& mut self) { extern "C-unwind" { fn CFRelease (cf : * mut c_void) ; } unsafe { CFRelease (self . ptr . as_ptr () . cast ()) } ; } }
};
}
