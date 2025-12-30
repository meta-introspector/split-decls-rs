// Generated macro for impl_83 (impl)
macro_rules! Depcrate_arch_all_memchrimpl_83 {
() => {
// Module: crate::arch::all::memchr
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for ThreeIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
