// Generated macro for impl_141 (impl)
macro_rules! Depcrate_visit_filterimpl_141 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_141"}
// Dependencies: {}
impl < N > FilterNode < N > for & FixedBitSet where FixedBitSet : VisitMap < N > , { fn include_node (& self , n : N) -> bool { self . is_visited (& n) } }
};
}
