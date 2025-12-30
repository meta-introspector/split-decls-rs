// Generated macro for impl_1724 (impl)
macro_rules! Depcrate_vec_spliceimpl_1724 {
() => {
// Module: crate::vec::splice
// Provides: {"impl_1724"}
// Dependencies: {}
# [stable (feature = "vec_splice" , since = "1.21.0")] impl < I : Iterator , A : Allocator > DoubleEndedIterator for Splice < '_ , I , A > { fn next_back (& mut self) -> Option < Self :: Item > { self . drain . next_back () } }
};
}
