// Generated macro for impl_78 (impl)
macro_rules! Depcrate_arch_all_memchrimpl_78 {
() => {
// Module: crate::arch::all::memchr
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for TwoIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
