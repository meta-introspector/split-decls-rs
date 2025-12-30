// Generated macro for impl_201 (impl)
macro_rules! Depcrate_retainedimpl_201 {
() => {
// Module: crate::retained
// Provides: {"impl_201"}
// Dependencies: {}
# [cfg (feature = "objc2")] impl < T : ? Sized + DispatchObject + objc2 :: Message > From < objc2 :: rc :: Retained < T > > for DispatchRetained < T > { # [doc = " Convert a [`objc2::rc::Retained`] into a [`DispatchRetained`]."] # [doc = ""] # [doc = " This only works if the type is a Dispatch object (implements the"] # [doc = " [`DispatchObject`] trait)."] # [doc = ""] # [doc = " This conversion is cost-free."] # [inline] fn from (obj : objc2 :: rc :: Retained < T >) -> Self { let ptr = objc2 :: rc :: Retained :: into_raw (obj) ; let ptr = NonNull :: new (ptr) . unwrap () ; unsafe { Self :: from_raw (ptr) } } }
};
}
