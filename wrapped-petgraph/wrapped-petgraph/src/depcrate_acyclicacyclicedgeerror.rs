// Generated macro for AcyclicEdgeError (enum)
macro_rules! Depcrate_acyclicAcyclicEdgeError {
() => {
// Module: crate::acyclic
// Provides: {"AcyclicEdgeError"}
// Dependencies: {}
# [doc = " An error that can occur during edge addition for acyclic graphs."] # [derive (Clone , Debug , PartialEq)] pub enum AcyclicEdgeError < N > { # [doc = " The edge would create a cycle."] Cycle (Cycle < N >) , # [doc = " The edge would create a self-loop."] SelfLoop , # [doc = " Could not successfully add the edge to the underlying graph."] InvalidEdge , }
};
}
