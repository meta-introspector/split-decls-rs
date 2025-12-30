// Generated macro for impl_176 (impl)
macro_rules! Depcrate_visit_filterimpl_176 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_176"}
// Dependencies: {}
impl < G , F > Iterator for EdgeFilteredNeighbors < '_ , G , F > where F : FilterEdge < G :: EdgeRef > , G : IntoEdges , { type Item = G :: NodeId ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; (& mut self . iter) . filter_map (move | edge | { if f . include_edge (edge) { Some (edge . target ()) } else { None } }) . next () } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
};
}
