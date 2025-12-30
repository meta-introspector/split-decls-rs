// Generated macro for impl_38 (impl)
macro_rules! Depcrate_arcimpl_38 {
() => {
// Module: crate::arc
// Provides: {"impl_38"}
// Dependencies: {}
impl < T : ? Sized > Clone for Arc < T > { # [doc = " Makes a clone of the `Arc` pointer."] # [doc = ""] # [doc = " This creates another pointer to the same allocation, increasing the"] # [doc = " strong reference count."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = ""] # [doc = " let _ = Arc::clone(&five);"] # [doc = " ```"] # [inline] fn clone (& self) -> Self { let old_size = self . inner () . strong . fetch_add (1 , Relaxed) ; if old_size > MAX_REFCOUNT { abort () ; } unsafe { Self :: from_inner (self . ptr) } } }
};
}
