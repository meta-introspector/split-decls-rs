// Generated macro for impl_51 (impl)
macro_rules! Depcrate_arcimpl_51 {
() => {
// Module: crate::arc
// Provides: {"impl_51"}
// Dependencies: {}
impl < T : ? Sized > Clone for Weak < T > { # [doc = " Makes a clone of the `Weak` pointer that points to the same allocation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::{Arc, Weak};"] # [doc = ""] # [doc = " let weak_five = Arc::downgrade(&Arc::new(5));"] # [doc = ""] # [doc = " let _ = Weak::clone(&weak_five);"] # [doc = " ```"] # [inline] fn clone (& self) -> Self { if let Some (inner) = self . inner () { let old_size = inner . weak . fetch_add (1 , Relaxed) ; if old_size > MAX_REFCOUNT { abort () ; } } Self { ptr : self . ptr } } }
};
}
