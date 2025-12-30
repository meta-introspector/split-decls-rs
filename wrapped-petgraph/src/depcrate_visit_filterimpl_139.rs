// Generated macro for impl_139 (impl)
macro_rules! Depcrate_visit_filterimpl_139 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_139"}
// Dependencies: {}
# [doc = " This filter includes the nodes that are contained in the set."] impl < N > FilterNode < N > for FixedBitSet where FixedBitSet : VisitMap < N > , { fn include_node (& self , n : N) -> bool { self . is_visited (& n) } }
};
}
