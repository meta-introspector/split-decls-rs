// Generated macro for impl_177 (impl)
macro_rules! Depcrate_arch_aarch64_neon_memchrimpl_177 {
() => {
// Module: crate::arch::aarch64::neon::memchr
// Provides: {"impl_177"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for TwoIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
