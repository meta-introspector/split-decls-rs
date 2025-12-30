// Generated macro for impl_344 (impl)
macro_rules! Depcrate_memchrimpl_344 {
() => {
// Module: crate::memchr
// Provides: {"impl_344"}
// Dependencies: {}
impl < 'h > Iterator for Memchr2 < 'h > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { unsafe { self . it . next (| s , e | memchr2_raw (self . needle1 , self . needle2 , s , e)) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
