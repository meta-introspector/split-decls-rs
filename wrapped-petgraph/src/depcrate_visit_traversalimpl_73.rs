// Generated macro for impl_73 (impl)
macro_rules! Depcrate_visit_traversalimpl_73 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_73"}
// Dependencies: {}
impl < G > Walker < G > for Dfs < G :: NodeId , G :: Map > where G : IntoNeighbors + Visitable , { type Item = G :: NodeId ; fn walk_next (& mut self , context : G) -> Option < Self :: Item > { self . next (context) } }
};
}
