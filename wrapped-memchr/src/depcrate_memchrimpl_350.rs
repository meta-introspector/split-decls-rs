// Generated macro for impl_350 (impl)
macro_rules! Depcrate_memchrimpl_350 {
() => {
// Module: crate::memchr
// Provides: {"impl_350"}
// Dependencies: {}
impl < 'h > DoubleEndedIterator for Memchr3 < 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | { memrchr3_raw (self . needle1 , self . needle2 , self . needle3 , s , e) }) } } }
};
}
