// Generated macro for impl_150 (impl)
macro_rules! Depcrate_visit_filterimpl_150 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'a , G , F > IntoNodeIdentifiers for & 'a NodeFiltered < G , F > where G : IntoNodeIdentifiers , F : FilterNode < G :: NodeId > , { type NodeIdentifiers = NodeFilteredNeighbors < 'a , G :: NodeIdentifiers , F > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { NodeFilteredNeighbors { include_source : true , iter : self . 0 . node_identifiers () , f : & self . 1 , } } }
};
}
