// Generated macro for impl_1237 (impl)
macro_rules! Depcrate_rcimpl_1237 {
() => {
// Module: crate::rc
// Provides: {"impl_1237"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator + Clone > Clone for Rc < T , A > { # [doc = " Makes a clone of the `Rc` pointer."] # [doc = ""] # [doc = " This creates another pointer to the same allocation, increasing the"] # [doc = " strong reference count."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Rc;"] # [doc = ""] # [doc = " let five = Rc::new(5);"] # [doc = ""] # [doc = " let _ = Rc::clone(&five);"] # [doc = " ```"] # [inline] fn clone (& self) -> Self { unsafe { self . inner () . inc_strong () ; Self :: from_inner_in (self . ptr , self . alloc . clone ()) } } }
};
}
