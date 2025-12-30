// Generated macro for NodeRef (trait)
macro_rules! Depcrate_visitNodeRef {
() => {
// Module: crate::visit
// Provides: {"NodeRef"}
// Dependencies: {}
# [doc = " A node reference."] pub trait NodeRef : Copy { type NodeId ; type Weight ; fn id (& self) -> Self :: NodeId ; fn weight (& self) -> & Self :: Weight ; }
};
}
