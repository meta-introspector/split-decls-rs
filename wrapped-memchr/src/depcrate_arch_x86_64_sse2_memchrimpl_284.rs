// Generated macro for impl_284 (impl)
macro_rules! Depcrate_arch_x86_64_sse2_memchrimpl_284 {
() => {
// Module: crate::arch::x86_64::sse2::memchr
// Provides: {"impl_284"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for TwoIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
