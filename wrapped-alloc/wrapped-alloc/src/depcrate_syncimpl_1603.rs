// Generated macro for impl_1603 (impl)
macro_rules! Depcrate_syncimpl_1603 {
() => {
// Module: crate::sync
// Provides: {"impl_1603"}
// Dependencies: {}
# [stable (feature = "arc_weak" , since = "1.4.0")] unsafe impl < # [may_dangle] T : ? Sized , A : Allocator > Drop for Weak < T , A > { # [doc = " Drops the `Weak` pointer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::{Arc, Weak};"] # [doc = ""] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl Drop for Foo {"] # [doc = "     fn drop(&mut self) {"] # [doc = "         println!(\"dropped!\");"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let foo = Arc::new(Foo);"] # [doc = " let weak_foo = Arc::downgrade(&foo);"] # [doc = " let other_weak_foo = Weak::clone(&weak_foo);"] # [doc = ""] # [doc = " drop(weak_foo);   // Doesn't print anything"] # [doc = " drop(foo);        // Prints \"dropped!\""] # [doc = ""] # [doc = " assert!(other_weak_foo.upgrade().is_none());"] # [doc = " ```"] fn drop (& mut self) { let inner = if let Some (inner) = self . inner () { inner } else { return } ; if inner . weak . fetch_sub (1 , Release) == 1 { acquire ! (inner . weak) ; debug_assert ! (! ptr :: addr_eq (self . ptr . as_ptr () , & STATIC_INNER_SLICE . inner) , "Arc/Weaks backed by a static should never be deallocated. \
                Likely decrement_strong_count or from_raw were called too many times." ,) ; unsafe { self . alloc . deallocate (self . ptr . cast () , Layout :: for_value_raw (self . ptr . as_ptr ())) } } } }
};
}
