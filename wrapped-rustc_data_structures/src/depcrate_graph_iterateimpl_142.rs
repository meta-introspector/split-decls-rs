// Generated macro for impl_142 (impl)
macro_rules! Depcrate_graph_iterateimpl_142 {
() => {
// Module: crate::graph::iterate
// Provides: {"impl_142"}
// Dependencies: {}
impl < G > TriColorVisitor < G > for CycleDetector where G : ? Sized + DirectedGraph , { type BreakVal = () ; fn node_examined (& mut self , _node : G :: Node , prior_status : Option < NodeStatus > ,) -> ControlFlow < Self :: BreakVal > { match prior_status { Some (NodeStatus :: Visited) => ControlFlow :: Break (()) , _ => ControlFlow :: Continue (()) , } } }
};
}
