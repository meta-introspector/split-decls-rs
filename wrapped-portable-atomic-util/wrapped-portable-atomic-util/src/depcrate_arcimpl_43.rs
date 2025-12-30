// Generated macro for impl_43 (impl)
macro_rules! Depcrate_arcimpl_43 {
() => {
// Module: crate::arc
// Provides: {"impl_43"}
// Dependencies: {}
impl < T : ? Sized > Drop for Arc < T > { # [doc = " Drops the `Arc`."] # [doc = ""] # [doc = " This will decrement the strong reference count. If the strong reference"] # [doc = " count reaches zero then the only other references (if any) are"] # [doc = " [`Weak`], so we `drop` the inner value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = ""] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl Drop for Foo {"] # [doc = "     fn drop(&mut self) {"] # [doc = "         println!(\"dropped!\");"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let foo = Arc::new(Foo);"] # [doc = " let foo2 = Arc::clone(&foo);"] # [doc = ""] # [doc = " drop(foo); // Doesn't print anything"] # [doc = " drop(foo2); // Prints \"dropped!\""] # [doc = " ```"] # [inline] fn drop (& mut self) { if self . inner () . strong . fetch_sub (1 , Release) != 1 { return ; } acquire ! (self . inner () . strong) ; unsafe { self . drop_slow () ; } } }
};
}
