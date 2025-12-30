// Generated macro for impl_1723 (impl)
macro_rules! Depcrate_vec_spliceimpl_1723 {
() => {
// Module: crate::vec::splice
// Provides: {"impl_1723"}
// Dependencies: {}
# [stable (feature = "vec_splice" , since = "1.21.0")] impl < I : Iterator , A : Allocator > Iterator for Splice < '_ , I , A > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . drain . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . drain . size_hint () } }
};
}
