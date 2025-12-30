// Generated macro for impl_75 (impl)
macro_rules! Depcrate_visit_traversalimpl_75 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_75"}
// Dependencies: {}
impl < G > Walker < G > for Bfs < G :: NodeId , G :: Map > where G : IntoNeighbors + Visitable , { type Item = G :: NodeId ; fn walk_next (& mut self , context : G) -> Option < Self :: Item > { self . next (context) } }
};
}
