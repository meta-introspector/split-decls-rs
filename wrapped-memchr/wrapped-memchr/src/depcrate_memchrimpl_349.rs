// Generated macro for impl_349 (impl)
macro_rules! Depcrate_memchrimpl_349 {
() => {
// Module: crate::memchr
// Provides: {"impl_349"}
// Dependencies: {}
impl < 'h > Iterator for Memchr3 < 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | { memchr3_raw (self . needle1 , self . needle2 , self . needle3 , s , e) }) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
