// Generated macro for impl_16 (impl)
macro_rules! Depcrate_boxedimpl_16 {
() => {
// Module: crate::boxed
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a , T : ? Sized > From < Box < 'a , T > > for Pin < Box < 'a , T > > { # [doc = " Converts a `Box<T>` into a `Pin<Box<T>>`."] # [doc = ""] # [doc = " This conversion does not allocate on the heap and happens in place."] fn from (boxed : Box < 'a , T >) -> Self { unsafe { Pin :: new_unchecked (boxed) } } }
};
}
