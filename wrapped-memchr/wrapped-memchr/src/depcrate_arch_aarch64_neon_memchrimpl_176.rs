// Generated macro for impl_176 (impl)
macro_rules! Depcrate_arch_aarch64_neon_memchrimpl_176 {
() => {
// Module: crate::arch::aarch64::neon::memchr
// Provides: {"impl_176"}
// Dependencies: {}
impl < 'a , 'h > Iterator for TwoIter < 'a , 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | self . searcher . find_raw (s , e)) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
