// Generated macro for impl_58 (impl)
macro_rules! Depcrate_baseimpl_58 {
() => {
// Module: crate::base
// Provides: {"impl_58"}
// Dependencies: {}
impl CFType { # [doc = " Attempt to downcast the type to that of type `T`."] # [doc = ""] # [doc = " This is the reference-variant. Use [`CFRetained::downcast`] if you"] # [doc = " want to convert a retained type. See also [`ConcreteType`] for more"] # [doc = " details on which types support being converted to."] # [doc = ""] # [doc = " [`CFRetained::downcast`]: crate::CFRetained::downcast"] # [doc (alias = "CFGetTypeID")] pub fn downcast_ref < T : ConcreteType > (& self) -> Option < & T > { if CFGetTypeID (Some (self)) == T :: type_id () { let ptr : * const Self = self ; let ptr : * const T = ptr . cast () ; let this : & T = unsafe { & * ptr } ; Some (this) } else { None } } # [doc = " Get the reference count of the object."] # [doc = ""] # [doc = " This function may be useful for debugging. You normally do not use"] # [doc = " this function otherwise."] # [doc = ""] # [doc = " Beware that some things (like `CFNumber`s, small `CFString`s etc.) may"] # [doc = " not have a normal retain count for optimization purposes, and can"] # [doc = " return `usize::MAX` in that case."] # [doc (alias = "CFGetRetainCount")] pub fn retain_count (& self) -> usize { CFGetRetainCount (Some (self)) as _ } }
};
}
