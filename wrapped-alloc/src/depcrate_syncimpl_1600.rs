// Generated macro for impl_1600 (impl)
macro_rules! Depcrate_syncimpl_1600 {
() => {
// Module: crate::sync
// Provides: {"impl_1600"}
// Dependencies: {}
# [stable (feature = "arc_weak" , since = "1.4.0")] impl < T : ? Sized , A : Allocator + Clone > Clone for Weak < T , A > { # [doc = " Makes a clone of the `Weak` pointer that points to the same allocation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::{Arc, Weak};"] # [doc = ""] # [doc = " let weak_five = Arc::downgrade(&Arc::new(5));"] # [doc = ""] # [doc = " let _ = Weak::clone(&weak_five);"] # [doc = " ```"] # [inline] fn clone (& self) -> Weak < T , A > { if let Some (inner) = self . inner () { let old_size = inner . weak . fetch_add (1 , Relaxed) ; if old_size > MAX_REFCOUNT { abort () ; } } Weak { ptr : self . ptr , alloc : self . alloc . clone () } } }
};
}
