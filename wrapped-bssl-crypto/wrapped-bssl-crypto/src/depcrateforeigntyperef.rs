// Generated macro for ForeignTypeRef (trait)
macro_rules! DepcrateForeignTypeRef {
() => {
// Module: crate
// Provides: {"ForeignTypeRef"}
// Dependencies: {}
# [doc = " A helper trait implemented by types which reference borrowed foreign types."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations of `ForeignTypeRef` must guarantee the following:"] # [doc = ""] # [doc = " - `Self::from_ptr(x).as_ptr() == x`"] # [doc = " - `Self::from_ptr_mut(x).as_ptr() == x`"] unsafe trait ForeignTypeRef : Sized { # [doc = " The raw C type."] type CType ; # [doc = " Constructs a shared instance of this type from its raw type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `ptr` must be a valid, immutable, instance of the type for the `'a` lifetime."] # [inline] unsafe fn from_ptr < 'a > (ptr : * mut Self :: CType) -> & 'a Self { debug_assert ! (! ptr . is_null ()) ; unsafe { & * (ptr as * mut _) } } # [doc = " Returns a raw pointer to the wrapped value."] # [inline] fn as_ptr (& self) -> * mut Self :: CType { self as * const _ as * mut _ } }
};
}
