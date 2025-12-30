// Generated macro for impl_1284 (impl)
macro_rules! Depcrate_rcimpl_1284 {
() => {
// Module: crate::rc
// Provides: {"impl_1284"}
// Dependencies: {}
# [stable (feature = "rc_weak" , since = "1.4.0")] impl < T : ? Sized , A : Allocator + Clone > Clone for Weak < T , A > { # [doc = " Makes a clone of the `Weak` pointer that points to the same allocation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::{Rc, Weak};"] # [doc = ""] # [doc = " let weak_five = Rc::downgrade(&Rc::new(5));"] # [doc = ""] # [doc = " let _ = Weak::clone(&weak_five);"] # [doc = " ```"] # [inline] fn clone (& self) -> Weak < T , A > { if let Some (inner) = self . inner () { inner . inc_weak () } Weak { ptr : self . ptr , alloc : self . alloc . clone () } } }
};
}
