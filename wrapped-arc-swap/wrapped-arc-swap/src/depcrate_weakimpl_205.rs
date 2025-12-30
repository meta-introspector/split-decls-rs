// Generated macro for impl_205 (impl)
macro_rules! Depcrate_weakimpl_205 {
() => {
// Module: crate::weak
// Provides: {"impl_205"}
// Dependencies: {}
unsafe impl < T > RefCnt for RcWeak < T > { type Base = T ; fn as_ptr (me : & Self) -> * mut T { if RcWeak :: ptr_eq (& RcWeak :: new () , me) { ptr :: null_mut () } else { RcWeak :: as_ptr (me) as * mut T } } fn into_ptr (me : Self) -> * mut T { if RcWeak :: ptr_eq (& RcWeak :: new () , & me) { ptr :: null_mut () } else { RcWeak :: into_raw (me) as * mut T } } unsafe fn from_ptr (ptr : * const T) -> Self { if ptr . is_null () { RcWeak :: new () } else { RcWeak :: from_raw (ptr) } } }
};
}
