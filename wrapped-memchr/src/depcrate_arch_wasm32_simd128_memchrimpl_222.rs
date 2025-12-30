// Generated macro for impl_222 (impl)
macro_rules! Depcrate_arch_wasm32_simd128_memchrimpl_222 {
() => {
// Module: crate::arch::wasm32::simd128::memchr
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'a , 'h > DoubleEndedIterator for ThreeIter < 'a , 'h > { # [inline] fn next_back (& mut self) -> Option < usize > { unsafe { self . it . next_back (| s , e | self . searcher . rfind_raw (s , e)) } } }
};
}
