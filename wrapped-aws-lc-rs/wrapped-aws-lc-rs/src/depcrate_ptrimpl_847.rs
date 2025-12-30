// Generated macro for impl_847 (impl)
macro_rules! Depcrate_ptrimpl_847 {
() => {
// Module: crate::ptr
// Provides: {"impl_847"}
// Dependencies: {}
impl < T > IntoPointer < * mut T > for * mut T { # [inline] fn into_pointer (self) -> Option < * mut T > { if self . is_null () { None } else { Some (self) } } }
};
}
