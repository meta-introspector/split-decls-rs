// Generated macro for impl_277 (impl)
macro_rules! Depcrate_arch_x86_64_sse2_memchrimpl_277 {
() => {
// Module: crate::arch::x86_64::sse2::memchr
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'a , 'h > Iterator for OneIter < 'a , 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | self . searcher . find_raw (s , e)) } } # [inline] fn count (self) -> usize { self . it . count (| s , e | { unsafe { self . searcher . count_raw (s , e) } }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
