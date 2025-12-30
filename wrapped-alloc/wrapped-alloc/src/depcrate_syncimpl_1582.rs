// Generated macro for impl_1582 (impl)
macro_rules! Depcrate_syncimpl_1582 {
() => {
// Module: crate::sync
// Provides: {"impl_1582"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator + Clone > Clone for Arc < T , A > { # [doc = " Makes a clone of the `Arc` pointer."] # [doc = ""] # [doc = " This creates another pointer to the same allocation, increasing the"] # [doc = " strong reference count."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = ""] # [doc = " let _ = Arc::clone(&five);"] # [doc = " ```"] # [inline] fn clone (& self) -> Arc < T , A > { let old_size = self . inner () . strong . fetch_add (1 , Relaxed) ; if old_size > MAX_REFCOUNT { abort () ; } unsafe { Self :: from_inner_in (self . ptr , self . alloc . clone ()) } } }
};
}
