// Generated macro for impl_210 (impl)
macro_rules! Depcrate_arch_wasm32_simd128_memchrimpl_210 {
() => {
// Module: crate::arch::wasm32::simd128::memchr
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for OneIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
