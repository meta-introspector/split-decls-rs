// Generated macro for impl_209 (impl)
macro_rules! Depcrate_arch_wasm32_simd128_memchrimpl_209 {
() => {
// Module: crate::arch::wasm32::simd128::memchr
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'a , 'h > Iterator for OneIter < 'a , 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | self . searcher . find_raw (s , e)) } } # [inline] fn count (self) -> usize { self . it . count (| s , e | { unsafe { self . searcher . count_raw (s , e) } }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
