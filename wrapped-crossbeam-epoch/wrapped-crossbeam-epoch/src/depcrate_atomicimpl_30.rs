// Generated macro for impl_30 (impl)
macro_rules! Depcrate_atomicimpl_30 {
() => {
// Module: crate::atomic
// Provides: {"impl_30"}
// Dependencies: {}
impl < T > Pointable for T { const ALIGN : usize = mem :: align_of :: < T > () ; type Init = T ; unsafe fn init (init : Self :: Init) -> * mut () { Box :: into_raw (Box :: new (init)) . cast :: < () > () } unsafe fn deref < 'a > (ptr : * mut ()) -> & 'a Self { unsafe { & * (ptr as * const T) } } unsafe fn deref_mut < 'a > (ptr : * mut ()) -> & 'a mut Self { unsafe { & mut * ptr . cast :: < T > () } } unsafe fn drop (ptr : * mut ()) { drop (unsafe { Box :: from_raw (ptr . cast :: < T > ()) }) ; } }
};
}
