// Generated macro for Node (struct)
macro_rules! Depcrate_graph_implNode {
() => {
// Module: crate::graph_impl
// Provides: {"Node"}
// Dependencies: {}
# [doc = " The graph's node type."] # [derive (Debug)] pub struct Node < N , Ix = DefaultIx > { # [doc = " Associated node data."] pub weight : N , # [doc = " Next edge in outgoing and incoming edge lists."] next : [EdgeIndex < Ix > ; 2] , }
};
}
