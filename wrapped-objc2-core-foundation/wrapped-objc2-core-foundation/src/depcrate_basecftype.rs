// Generated macro for CFType (struct)
macro_rules! Depcrate_baseCFType {
() => {
// Module: crate::base
// Provides: {"CFType"}
// Dependencies: {}
# [doc = " An instance of a Core Foundation type."] # [doc = ""] # [doc = " This is meant to be used behind a reference. In the future, this will be"] # [doc = " defined as an [`extern type`][RFC-1861]."] # [doc = ""] # [doc = " All Core Foundation types [`Deref`](std::ops::Deref) to this type (it can"] # [doc = " be considered the \"root\" type)."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cftype?language=objc)."] # [doc = ""] # [doc = " [RFC-1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html"] # [repr (C)] pub struct CFType { inner : [u8 ; 0] , _p : UnsafeCell < PhantomData < (* const UnsafeCell < () > , PhantomPinned) > > , }
};
}
