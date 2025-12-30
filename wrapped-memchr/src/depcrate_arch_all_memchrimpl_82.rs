// Generated macro for impl_82 (impl)
macro_rules! Depcrate_arch_all_memchrimpl_82 {
() => {
// Module: crate::arch::all::memchr
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a , 'h > Iterator for ThreeIter < 'a , 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | self . searcher . find_raw (s , e)) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
