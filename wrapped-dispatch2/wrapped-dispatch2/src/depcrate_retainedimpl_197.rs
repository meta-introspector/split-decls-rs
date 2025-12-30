// Generated macro for impl_197 (impl)
macro_rules! Depcrate_retainedimpl_197 {
() => {
// Module: crate::retained
// Provides: {"impl_197"}
// Dependencies: {}
impl < T : ? Sized + DispatchObject > Clone for DispatchRetained < T > { # [doc = " Retain the object, increasing its reference count."] # [doc = ""] # [doc = " This calls [`DispatchObject::retain`] internally."] # [doc (alias = "dispatch_retain")] # [doc (alias = "retain")] # [inline] fn clone (& self) -> Self { self . retain () } }
};
}
