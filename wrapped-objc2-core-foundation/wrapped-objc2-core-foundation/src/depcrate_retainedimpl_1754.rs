// Generated macro for impl_1754 (impl)
macro_rules! Depcrate_retainedimpl_1754 {
() => {
// Module: crate::retained
// Provides: {"impl_1754"}
// Dependencies: {}
# [cfg (feature = "objc2")] impl < T : ? Sized + objc2 :: Message > From < CFRetained < T > > for objc2 :: rc :: Retained < T > { # [doc = " Convert a [`CFRetained`] into a [`objc2::rc::Retained`]."] # [doc = ""] # [doc = " This conversion is cost-free, since CoreFoundation types are fully"] # [doc = " interoperable with Objective-C retain/release message sending."] # [inline] fn from (obj : CFRetained < T >) -> Self { let ptr = ManuallyDrop :: new (obj) . ptr ; unsafe { Self :: from_raw (ptr . as_ptr ()) } . unwrap () } }
};
}
