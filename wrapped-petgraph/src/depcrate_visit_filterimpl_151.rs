// Generated macro for impl_151 (impl)
macro_rules! Depcrate_visit_filterimpl_151 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a , G , F > IntoNodeReferences for & 'a NodeFiltered < G , F > where G : IntoNodeReferences , F : FilterNode < G :: NodeId > , { type NodeRef = G :: NodeRef ; type NodeReferences = NodeFilteredNodes < 'a , G :: NodeReferences , F > ; fn node_references (self) -> Self :: NodeReferences { NodeFilteredNodes { include_source : true , iter : self . 0 . node_references () , f : & self . 1 , } } }
};
}
