// Generated macro for impl_142 (impl)
macro_rules! Depcrate_ref_cntimpl_142 {
() => {
// Module: crate::ref_cnt
// Provides: {"impl_142"}
// Dependencies: {}
unsafe impl < T > RefCnt for Arc < T > { type Base = T ; fn into_ptr (me : Arc < T >) -> * mut T { Arc :: into_raw (me) as * mut T } fn as_ptr (me : & Arc < T >) -> * mut T { let ptr = Arc :: into_raw (unsafe { ptr :: read (me) }) ; let ptr = ptr as * mut T ; mem :: forget (unsafe { Arc :: from_raw (ptr) }) ; ptr } unsafe fn from_ptr (ptr : * const T) -> Arc < T > { Arc :: from_raw (ptr) } }
};
}
