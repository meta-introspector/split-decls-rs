// Generated macro for ForeignTypeRef (trait)
macro_rules! DepcrateForeignTypeRef {
() => {
// Module: crate
// Provides: {"ForeignTypeRef"}
// Dependencies: {}
# [doc = " A trait implemented by types which reference borrowed foreign types."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations of `ForeignTypeRef` must guarantee the following:"] # [doc = ""] # [doc = " - `Self::from_ptr(x).as_ptr() == x`"] # [doc = " - `Self::from_mut_ptr(x).as_ptr() == x`"] pub unsafe trait ForeignTypeRef : Sized { # [doc = " The raw C type."] type CType ; # [doc = " Constructs a shared instance of this type from its raw type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `ptr` must be a valid, immutable, instance of the type for the `'a` lifetime."] # [inline] unsafe fn from_ptr < 'a > (ptr : * mut Self :: CType) -> & 'a Self { debug_assert ! (! ptr . is_null ()) ; & * (ptr as * mut _) } # [doc = " Constructs a mutable reference of this type from its raw type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `ptr` must be a valid, unique, instance of the type for the `'a` lifetime."] # [inline] unsafe fn from_ptr_mut < 'a > (ptr : * mut Self :: CType) -> & 'a mut Self { debug_assert ! (! ptr . is_null ()) ; & mut * (ptr as * mut _) } # [doc = " Returns a raw pointer to the wrapped value."] # [inline] fn as_ptr (& self) -> * mut Self :: CType { self as * const _ as * mut _ } }
};
}
