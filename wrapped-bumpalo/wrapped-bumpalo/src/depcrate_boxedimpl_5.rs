// Generated macro for impl_5 (impl)
macro_rules! Depcrate_boxedimpl_5 {
() => {
// Module: crate::boxed
// Provides: {"impl_5"}
// Dependencies: {}
impl < 'a , T > Box < 'a , T > { # [doc = " Allocates memory on the heap and then places `x` into it."] # [doc = ""] # [doc = " This doesn't actually allocate if `T` is zero-sized."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::{Bump, boxed::Box};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let five = Box::new_in(5, &b);"] # [doc = " ```"] # [inline (always)] pub fn new_in (x : T , a : & 'a Bump) -> Box < 'a , T > { Box (a . alloc (x)) } # [doc = " Constructs a new `Pin<Box<T>>`. If `T` does not implement `Unpin`, then"] # [doc = " `x` will be pinned in memory and unable to be moved."] # [inline (always)] pub fn pin_in (x : T , a : & 'a Bump) -> Pin < Box < 'a , T > > { Box (a . alloc (x)) . into () } # [doc = " Consumes the `Box`, returning the wrapped value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::{Bump, boxed::Box};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let hello = Box::new_in(\"hello\".to_owned(), &b);"] # [doc = " assert_eq!(Box::into_inner(hello), \"hello\");"] # [doc = " ```"] pub fn into_inner (b : Box < 'a , T >) -> T { unsafe { core :: ptr :: read (Box :: into_raw (b)) } } }
};
}
