// Generated macro for impl_73 (impl)
macro_rules! Depcrate_arch_all_memchrimpl_73 {
() => {
// Module: crate::arch::all::memchr
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for OneIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
