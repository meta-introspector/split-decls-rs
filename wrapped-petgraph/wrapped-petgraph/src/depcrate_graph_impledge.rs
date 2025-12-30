// Generated macro for Edge (struct)
macro_rules! Depcrate_graph_implEdge {
() => {
// Module: crate::graph_impl
// Provides: {"Edge"}
// Dependencies: {}
# [doc = " The graph's edge type."] # [derive (Debug)] pub struct Edge < E , Ix = DefaultIx > { # [doc = " Associated edge data."] pub weight : E , # [doc = " Next edge in outgoing and incoming edge lists."] next : [EdgeIndex < Ix > ; 2] , # [doc = " Start and End node index"] node : [NodeIndex < Ix > ; 2] , }
};
}
