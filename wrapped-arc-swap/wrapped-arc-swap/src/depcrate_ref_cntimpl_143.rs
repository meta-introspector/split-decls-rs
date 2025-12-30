// Generated macro for impl_143 (impl)
macro_rules! Depcrate_ref_cntimpl_143 {
() => {
// Module: crate::ref_cnt
// Provides: {"impl_143"}
// Dependencies: {}
unsafe impl < T > RefCnt for Rc < T > { type Base = T ; fn into_ptr (me : Rc < T >) -> * mut T { Rc :: into_raw (me) as * mut T } fn as_ptr (me : & Rc < T >) -> * mut T { let ptr = Rc :: into_raw (unsafe { ptr :: read (me) }) ; let ptr = ptr as * mut T ; mem :: forget (unsafe { Rc :: from_raw (ptr) }) ; ptr } unsafe fn from_ptr (ptr : * const T) -> Rc < T > { Rc :: from_raw (ptr) } }
};
}
