// Generated macro for macro_106 (macro)
macro_rules! Depcrate_visitmacro_106 {
() => {
// Module: crate::visit
// Provides: {"macro_106"}
// Dependencies: {}
trait_template ! { # [doc = " Access to the sequence of the graph’s nodes"] pub trait IntoNodeReferences : Data + IntoNodeIdentifiers { @ section type type NodeRef : NodeRef < NodeId = Self :: NodeId , Weight = Self :: NodeWeight >; type NodeReferences : Iterator < Item = Self :: NodeRef >; @ section self fn node_references (self) -> Self :: NodeReferences ; } }
};
}
