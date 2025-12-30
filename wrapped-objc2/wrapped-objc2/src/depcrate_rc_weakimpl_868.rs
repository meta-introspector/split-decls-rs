// Generated macro for impl_868 (impl)
macro_rules! Depcrate_rc_weakimpl_868 {
() => {
// Module: crate::rc::weak
// Provides: {"impl_868"}
// Dependencies: {}
impl < T : ? Sized > Drop for Weak < T > { # [doc = " Destroys the weak pointer."] # [doc (alias = "objc_destroyWeak")] # [inline] fn drop (& mut self) { unsafe { ffi :: objc_destroyWeak (self . inner . get ()) } } }
};
}
