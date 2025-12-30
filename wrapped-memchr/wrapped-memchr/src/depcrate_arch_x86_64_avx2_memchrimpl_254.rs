// Generated macro for impl_254 (impl)
macro_rules! Depcrate_arch_x86_64_avx2_memchrimpl_254 {
() => {
// Module: crate::arch::x86_64::avx2::memchr
// Provides: {"impl_254"}
// Dependencies: {}
impl < 'a , 'h > Iterator for TwoIter < 'a , 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | self . searcher . find_raw (s , e)) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
