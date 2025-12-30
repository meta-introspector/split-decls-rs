// Generated macro for impl_142 (impl)
macro_rules! Depcrate_visit_filterimpl_142 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_142"}
// Dependencies: {}
impl < N , S > FilterNode < N > for & HashSet < N , S > where HashSet < N , S > : VisitMap < N > , { fn include_node (& self , n : N) -> bool { self . is_visited (& n) } }
};
}
