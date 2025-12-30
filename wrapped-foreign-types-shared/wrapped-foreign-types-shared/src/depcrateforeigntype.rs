// Generated macro for ForeignType (trait)
macro_rules! DepcrateForeignType {
() => {
// Module: crate
// Provides: {"ForeignType"}
// Dependencies: {}
# [doc = " A type implemented by wrappers over foreign types."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations of `ForeignType` must guarantee the following:"] # [doc = " - `Self::from_ptr(x).as_ptr() == x`"] # [doc = " - `Self::from_ptr(x).into_ptr(x) == x`"] # [doc = " - `Self::from_ptr(x).deref().as_ptr(x) == x`"] # [doc = " - `Self::from_ptr(x).deref_mut().as_ptr(x) == x`"] # [doc = " - `Self::from_ptr(x).as_ref().as_ptr(x) == x`"] # [doc = " - `Self::from_ptr(x).as_mut().as_ptr(x) == x`"] pub unsafe trait ForeignType : Sized { # [doc = " The raw C type."] type CType ; # [doc = " The type representing a reference to this type."] type Ref : ForeignTypeRef < CType = Self :: CType > ; # [doc = " Constructs an instance of this type from its raw type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `ptr` must be a valid, owned instance of the native type."] unsafe fn from_ptr (ptr : * mut Self :: CType) -> Self ; # [doc = " Returns a raw pointer to the wrapped value."] fn as_ptr (& self) -> * mut Self :: CType ; # [doc = " Consumes the wrapper and returns the raw pointer."] # [inline] fn into_ptr (self) -> * mut Self :: CType { let ptr = self . as_ptr () ; mem :: forget (self) ; ptr } }
};
}
