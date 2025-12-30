// Generated macro for impl_290 (impl)
macro_rules! Depcrate_arch_x86_64_sse2_memchrimpl_290 {
() => {
// Module: crate::arch::x86_64::sse2::memchr
// Provides: {"impl_290"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for ThreeIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
