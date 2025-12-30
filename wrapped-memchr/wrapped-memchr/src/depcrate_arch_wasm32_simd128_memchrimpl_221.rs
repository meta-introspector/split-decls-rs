// Generated macro for impl_221 (impl)
macro_rules! Depcrate_arch_wasm32_simd128_memchrimpl_221 {
() => {
// Module: crate::arch::wasm32::simd128::memchr
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a , 'h > Iterator for ThreeIter < 'a , 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | self . searcher . find_raw (s , e)) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
