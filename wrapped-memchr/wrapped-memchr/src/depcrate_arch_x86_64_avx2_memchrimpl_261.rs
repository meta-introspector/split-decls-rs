// Generated macro for impl_261 (impl)
macro_rules! Depcrate_arch_x86_64_avx2_memchrimpl_261 {
() => {
// Module: crate::arch::x86_64::avx2::memchr
// Provides: {"impl_261"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for ThreeIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
