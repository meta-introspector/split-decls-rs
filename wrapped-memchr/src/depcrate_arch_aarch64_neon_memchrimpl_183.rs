// Generated macro for impl_183 (impl)
macro_rules! Depcrate_arch_aarch64_neon_memchrimpl_183 {
() => {
// Module: crate::arch::aarch64::neon::memchr
// Provides: {"impl_183"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for ThreeIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
