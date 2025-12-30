// Generated macro for impl_13 (impl)
macro_rules! Depcrate___ns_macro_helpers_cachedimpl_13 {
() => {
// Module: crate::__ns_macro_helpers::cached
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : Message > CachedRetained < T > { # [doc = " Returns the cached object. If no object is yet cached, creates one"] # [doc = " from the given closure and stores it."] # [inline] pub fn get (& self , f : impl FnOnce () -> Retained < T >) -> & 'static T { let ptr = self . ptr . load (Ordering :: SeqCst) ; unsafe { ptr . as_ref () } . unwrap_or_else (| | { let s = ManuallyDrop :: new (f ()) ; let ptr = Retained :: as_ptr (& s) ; self . ptr . store (ptr as * mut T , Ordering :: SeqCst) ; unsafe { ptr . as_ref () . unwrap_unchecked () } }) } }
};
}
