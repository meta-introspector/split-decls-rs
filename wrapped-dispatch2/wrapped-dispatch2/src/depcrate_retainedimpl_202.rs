// Generated macro for impl_202 (impl)
macro_rules! Depcrate_retainedimpl_202 {
() => {
// Module: crate::retained
// Provides: {"impl_202"}
// Dependencies: {}
# [cfg (feature = "objc2")] impl < T : ? Sized + DispatchObject + objc2 :: Message > From < DispatchRetained < T > > for objc2 :: rc :: Retained < T > { # [doc = " Convert a [`DispatchRetained`] into a [`objc2::rc::Retained`]."] # [doc = ""] # [doc = " This conversion is cost-free, since Dispatch objects are fully"] # [doc = " interoperable with Objective-C retain/release message sending."] # [inline] fn from (obj : DispatchRetained < T >) -> Self { let ptr = DispatchRetained :: into_raw (obj) ; unsafe { Self :: from_raw (ptr . as_ptr ()) } . unwrap () } }
};
}
