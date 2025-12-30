// Generated macro for impl_339 (impl)
macro_rules! Depcrate_memchrimpl_339 {
() => {
// Module: crate::memchr
// Provides: {"impl_339"}
// Dependencies: {}
impl < 'h > Iterator for Memchr < 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | memchr_raw (self . needle1 , s , e)) } } # [inline] fn count (self) -> usize { self . it . count (| s , e | { unsafe { count_raw (self . needle1 , s , e) } }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
