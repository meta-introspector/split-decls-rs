// Generated macro for impl_74 (impl)
macro_rules! Depcrate_visit_traversalimpl_74 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_74"}
// Dependencies: {}
impl < G > Walker < G > for DfsPostOrder < G :: NodeId , G :: Map > where G : IntoNeighbors + Visitable , { type Item = G :: NodeId ; fn walk_next (& mut self , context : G) -> Option < Self :: Item > { self . next (context) } }
};
}
