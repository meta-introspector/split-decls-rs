// Generated macro for impl_1747 (impl)
macro_rules! Depcrate_retainedimpl_1747 {
() => {
// Module: crate::retained
// Provides: {"impl_1747"}
// Dependencies: {}
impl < T : Type > Clone for CFRetained < T > { # [doc = " Retain the type, increasing its reference count."] # [doc = ""] # [doc = " This calls [`Type::retain`] internally."] # [doc (alias = "CFRetain")] # [doc (alias = "retain")] # [inline] fn clone (& self) -> Self { self . retain () } }
};
}
