// Generated macro for impl_140 (impl)
macro_rules! Depcrate_visit_filterimpl_140 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_140"}
// Dependencies: {}
# [doc = " This filter includes the nodes that are contained in the set."] impl < N , S > FilterNode < N > for HashSet < N , S > where HashSet < N , S > : VisitMap < N > , { fn include_node (& self , n : N) -> bool { self . is_visited (& n) } }
};
}
