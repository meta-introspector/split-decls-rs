// Generated macro for impl_739 (impl)
macro_rules! Depcrate_rc_allocated_partial_initimpl_739 {
() => {
// Module: crate::rc::allocated_partial_init
// Provides: {"impl_739"}
// Dependencies: {}
impl < T : ? Sized + Message > PartialInit < T > { # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure the pointer is NULL, or that the given object"] # [doc = " is allocated, has +1 retain count, and that the class' instance"] # [doc = " variables have been initialized."] # [inline] pub (crate) unsafe fn new (ptr : * mut T) -> Self { Self { ptr , p : PhantomData , p_auto_traits : PhantomData , } } # [doc = " Returns a raw pointer to the object."] # [doc = ""] # [doc = " The pointer is valid for at least as long as the `PartialInit` is"] # [doc = " held."] # [doc = ""] # [doc = " See [`PartialInit::as_mut_ptr`] for the mutable equivalent."] # [doc = ""] # [doc = " This is an associated method, and must be called as"] # [doc = " `PartialInit::as_ptr(obj)`."] # [inline] pub fn as_ptr (this : & Self) -> * const T { this . ptr } # [doc = " Returns a raw mutable pointer to the object."] # [doc = ""] # [doc = " The pointer is valid for at least as long as the `PartialInit` is"] # [doc = " held."] # [doc = ""] # [doc = " See [`PartialInit::as_ptr`] for the immutable equivalent."] # [doc = ""] # [doc = " This is an associated method, and must be called as"] # [doc = " `PartialInit::as_mut_ptr(obj)`."] # [inline] # [allow (unknown_lints)] # [allow (clippy :: needless_pass_by_ref_mut)] pub fn as_mut_ptr (this : & mut Self) -> * mut T { this . ptr as * mut T } # [inline] pub (crate) fn into_ptr (this : Self) -> * mut T { let this = ManuallyDrop :: new (this) ; this . ptr as * mut T } }
};
}
