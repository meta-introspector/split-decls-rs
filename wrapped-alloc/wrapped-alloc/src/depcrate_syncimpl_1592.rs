// Generated macro for impl_1592 (impl)
macro_rules! Depcrate_syncimpl_1592 {
() => {
// Module: crate::sync
// Provides: {"impl_1592"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < # [may_dangle] T : ? Sized , A : Allocator > Drop for Arc < T , A > { # [doc = " Drops the `Arc`."] # [doc = ""] # [doc = " This will decrement the strong reference count. If the strong reference"] # [doc = " count reaches zero then the only other references (if any) are"] # [doc = " [`Weak`], so we `drop` the inner value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl Drop for Foo {"] # [doc = "     fn drop(&mut self) {"] # [doc = "         println!(\"dropped!\");"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let foo  = Arc::new(Foo);"] # [doc = " let foo2 = Arc::clone(&foo);"] # [doc = ""] # [doc = " drop(foo);    // Doesn't print anything"] # [doc = " drop(foo2);   // Prints \"dropped!\""] # [doc = " ```"] # [inline] fn drop (& mut self) { if self . inner () . strong . fetch_sub (1 , Release) != 1 { return ; } acquire ! (self . inner () . strong) ; debug_assert ! (! ptr :: addr_eq (self . ptr . as_ptr () , & STATIC_INNER_SLICE . inner) , "Arcs backed by a static should never reach a strong count of 0. \
            Likely decrement_strong_count or from_raw were called too many times." ,) ; unsafe { self . drop_slow () ; } } }
};
}
