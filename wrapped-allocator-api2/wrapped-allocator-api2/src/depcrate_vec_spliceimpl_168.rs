// Generated macro for impl_168 (impl)
macro_rules! Depcrate_vec_spliceimpl_168 {
() => {
// Module: crate::vec::splice
// Provides: {"impl_168"}
// Dependencies: {}
impl < I : Iterator , A : Allocator > Iterator for Splice < '_ , I , A > { type Item = I :: Item ; # [inline (always)] fn next (& mut self) -> Option < Self :: Item > { self . drain . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . drain . size_hint () } }
};
}
