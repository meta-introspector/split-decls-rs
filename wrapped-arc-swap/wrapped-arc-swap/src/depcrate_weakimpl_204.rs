// Generated macro for impl_204 (impl)
macro_rules! Depcrate_weakimpl_204 {
() => {
// Module: crate::weak
// Provides: {"impl_204"}
// Dependencies: {}
unsafe impl < T > RefCnt for Weak < T > { type Base = T ; fn as_ptr (me : & Self) -> * mut T { if Weak :: ptr_eq (& Weak :: new () , me) { ptr :: null_mut () } else { Weak :: as_ptr (me) as * mut T } } fn into_ptr (me : Self) -> * mut T { if Weak :: ptr_eq (& Weak :: new () , & me) { ptr :: null_mut () } else { Weak :: into_raw (me) as * mut T } } unsafe fn from_ptr (ptr : * const T) -> Self { if ptr . is_null () { Weak :: new () } else { Weak :: from_raw (ptr) } } }
};
}
