// Generated macro for impl_869 (impl)
macro_rules! Depcrate_rc_weakimpl_869 {
() => {
// Module: crate::rc::weak
// Provides: {"impl_869"}
// Dependencies: {}
impl < T : Message > Clone for Weak < T > { # [doc = " Make a clone of the weak pointer that points to the same object."] # [doc (alias = "objc_copyWeak")] fn clone (& self) -> Self { let ptr = Box :: new (UnsafeCell :: new (ptr :: null_mut ())) ; unsafe { ffi :: objc_copyWeak (ptr . get () , self . inner . get ()) } ; Self { inner : ptr , item : PhantomData , } } }
};
}
