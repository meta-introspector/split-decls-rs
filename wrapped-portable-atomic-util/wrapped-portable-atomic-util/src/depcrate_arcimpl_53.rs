// Generated macro for impl_53 (impl)
macro_rules! Depcrate_arcimpl_53 {
() => {
// Module: crate::arc
// Provides: {"impl_53"}
// Dependencies: {}
impl < T : ? Sized > Drop for Weak < T > { # [doc = " Drops the `Weak` pointer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::{Arc, Weak};"] # [doc = ""] # [doc = " struct Foo;"] # [doc = ""] # [doc = " impl Drop for Foo {"] # [doc = "     fn drop(&mut self) {"] # [doc = "         println!(\"dropped!\");"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let foo = Arc::new(Foo);"] # [doc = " let weak_foo = Arc::downgrade(&foo);"] # [doc = " let other_weak_foo = Weak::clone(&weak_foo);"] # [doc = ""] # [doc = " drop(weak_foo); // Doesn't print anything"] # [doc = " drop(foo); // Prints \"dropped!\""] # [doc = ""] # [doc = " assert!(other_weak_foo.upgrade().is_none());"] # [doc = " ```"] fn drop (& mut self) { let inner = if let Some (inner) = self . inner () { inner } else { return } ; if inner . weak . fetch_sub (1 , Release) == 1 { acquire ! (inner . weak) ; let ptr = self . ptr . as_ptr () as * mut ArcInner < mem :: ManuallyDrop < T > > ; drop (unsafe { Box :: from_raw (ptr) }) ; } } }
};
}
