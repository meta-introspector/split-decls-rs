// Generated macro for impl_1283 (impl)
macro_rules! Depcrate_rcimpl_1283 {
() => {
// Module: crate::rc
// Provides: {"impl_1283"}
// Dependencies: {}
# [stable (feature = "rc_weak" , since = "1.4.0")] unsafe impl < # [may_dangle] T : ? Sized , A : Allocator > Drop for Weak < T , A > { # [doc = " Drops the `Weak` pointer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::{Rc, Weak};"] # [doc = ""] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl Drop for Foo {"] # [doc = "     fn drop(&mut self) {"] # [doc = "         println!(\"dropped!\");"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let foo = Rc::new(Foo);"] # [doc = " let weak_foo = Rc::downgrade(&foo);"] # [doc = " let other_weak_foo = Weak::clone(&weak_foo);"] # [doc = ""] # [doc = " drop(weak_foo);   // Doesn't print anything"] # [doc = " drop(foo);        // Prints \"dropped!\""] # [doc = ""] # [doc = " assert!(other_weak_foo.upgrade().is_none());"] # [doc = " ```"] fn drop (& mut self) { let inner = if let Some (inner) = self . inner () { inner } else { return } ; inner . dec_weak () ; if inner . weak () == 0 { unsafe { self . alloc . deallocate (self . ptr . cast () , Layout :: for_value_raw (self . ptr . as_ptr ())) ; } } } }
};
}
