// Generated macro for impl_345 (impl)
macro_rules! Depcrate_memchrimpl_345 {
() => {
// Module: crate::memchr
// Provides: {"impl_345"}
// Dependencies: {}
impl < 'h > DoubleEndedIterator for Memchr2 < 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | { memrchr2_raw (self . needle1 , self . needle2 , s , e) }) } } }
};
}
