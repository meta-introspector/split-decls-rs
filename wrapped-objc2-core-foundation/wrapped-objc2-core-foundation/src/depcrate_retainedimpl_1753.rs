// Generated macro for impl_1753 (impl)
macro_rules! Depcrate_retainedimpl_1753 {
() => {
// Module: crate::retained
// Provides: {"impl_1753"}
// Dependencies: {}
# [cfg (feature = "objc2")] impl < T : ? Sized + Type + objc2 :: Message > From < objc2 :: rc :: Retained < T > > for CFRetained < T > { # [doc = " Convert a [`objc2::rc::Retained`] into a [`CFRetained`]."] # [doc = ""] # [doc = " This only works if the type is a CoreFoundation type (implements the"] # [doc = " [`Type`] trait)."] # [doc = ""] # [doc = " This conversion is cost-free."] # [inline] fn from (obj : objc2 :: rc :: Retained < T >) -> Self { let ptr = objc2 :: rc :: Retained :: into_raw (obj) ; let ptr = NonNull :: new (ptr) . unwrap () ; unsafe { Self :: from_raw (ptr) } } }
};
}
