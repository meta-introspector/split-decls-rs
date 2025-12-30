// Generated macro for impl_76 (impl)
macro_rules! Depcrate_visit_traversalimpl_76 {
() => {
// Module: crate::visit::traversal
// Provides: {"impl_76"}
// Dependencies: {}
impl < G > Walker < G > for Topo < G :: NodeId , G :: Map > where G : IntoNeighborsDirected + Visitable , { type Item = G :: NodeId ; fn walk_next (& mut self , context : G) -> Option < Self :: Item > { self . next (context) } }
};
}
