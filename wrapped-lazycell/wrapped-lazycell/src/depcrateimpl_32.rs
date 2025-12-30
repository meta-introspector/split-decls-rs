// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < T : Clone > Clone for AtomicLazyCell < T > { # [doc = " Create a clone of this `AtomicLazyCell`"] # [doc = ""] # [doc = " If self has not been initialized, returns an uninitialized `AtomicLazyCell`"] # [doc = " otherwise returns an `AtomicLazyCell` already initialized with a clone of the"] # [doc = " contents of self."] fn clone (& self) -> AtomicLazyCell < T > { self . borrow () . map_or (Self :: NONE , | v | AtomicLazyCell { inner : UnsafeCell :: new (Some (v . clone ())) , state : AtomicUsize :: new (SOME) , }) } }
};
}
