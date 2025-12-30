// Generated macro for impl_340 (impl)
macro_rules! Depcrate_memchrimpl_340 {
() => {
// Module: crate::memchr
// Provides: {"impl_340"}
// Dependencies: {}
impl < 'h > DoubleEndedIterator for Memchr < 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | memrchr_raw (self . needle1 , s , e)) } } }
};
}
