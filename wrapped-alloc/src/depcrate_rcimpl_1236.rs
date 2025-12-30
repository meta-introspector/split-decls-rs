// Generated macro for impl_1236 (impl)
macro_rules! Depcrate_rcimpl_1236 {
() => {
// Module: crate::rc
// Provides: {"impl_1236"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < # [may_dangle] T : ? Sized , A : Allocator > Drop for Rc < T , A > { # [doc = " Drops the `Rc`."] # [doc = ""] # [doc = " This will decrement the strong reference count. If the strong reference"] # [doc = " count reaches zero then the only other references (if any) are"] # [doc = " [`Weak`], so we `drop` the inner value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Rc;"] # [doc = ""] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl Drop for Foo {"] # [doc = "     fn drop(&mut self) {"] # [doc = "         println!(\"dropped!\");"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let foo  = Rc::new(Foo);"] # [doc = " let foo2 = Rc::clone(&foo);"] # [doc = ""] # [doc = " drop(foo);    // Doesn't print anything"] # [doc = " drop(foo2);   // Prints \"dropped!\""] # [doc = " ```"] # [inline] fn drop (& mut self) { unsafe { self . inner () . dec_strong () ; if self . inner () . strong () == 0 { self . drop_slow () ; } } } }
};
}
