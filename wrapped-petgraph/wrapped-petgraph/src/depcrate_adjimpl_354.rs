// Generated macro for impl_354 (impl)
macro_rules! Depcrate_adjimpl_354 {
() => {
// Module: crate::adj
// Provides: {"impl_354"}
// Dependencies: {}
impl < Ix : IndexType , E > visit :: IntoNodeReferences for & List < E , Ix > { type NodeRef = NodeIndex < Ix > ; type NodeReferences = NodeIndices < Ix > ; fn node_references (self) -> Self :: NodeReferences { self . node_indices () } }
};
}
