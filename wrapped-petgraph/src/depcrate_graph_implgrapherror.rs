// Generated macro for GraphError (enum)
macro_rules! Depcrate_graph_implGraphError {
() => {
// Module: crate::graph_impl
// Provides: {"GraphError"}
// Dependencies: {}
# [doc = " The error type for fallible `Graph` & `StableGraph` operations."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum GraphError { # [doc = " The Graph is at the maximum number of nodes for its index."] NodeIxLimit , # [doc = " The Graph is at the maximum number of edges for its index."] EdgeIxLimit , # [doc = " The node with the specified index is missing from the graph."] NodeMissed (usize) , # [doc = " Node indices out of bounds."] NodeOutBounds , }
};
}
