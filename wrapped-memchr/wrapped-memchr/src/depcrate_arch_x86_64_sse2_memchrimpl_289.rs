// Generated macro for impl_289 (impl)
macro_rules! Depcrate_arch_x86_64_sse2_memchrimpl_289 {
() => {
// Module: crate::arch::x86_64::sse2::memchr
// Provides: {"impl_289"}
// Dependencies: {}
impl < 'a , 'h > Iterator for ThreeIter < 'a , 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | self . searcher . find_raw (s , e)) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
