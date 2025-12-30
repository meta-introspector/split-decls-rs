// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < T : Clone > Clone for LazyCell < T > { # [doc = " Create a clone of this `LazyCell`"] # [doc = ""] # [doc = " If self has not been initialized, returns an uninitialized `LazyCell`"] # [doc = " otherwise returns a `LazyCell` already initialized with a clone of the"] # [doc = " contents of self."] fn clone (& self) -> LazyCell < T > { LazyCell { inner : UnsafeCell :: new (self . borrow () . map (Clone :: clone)) } } }
};
}
