// Generated macro for Neighbors (struct)
macro_rules! Depcrate_graph_implNeighbors {
() => {
// Module: crate::graph_impl
// Provides: {"Neighbors"}
// Dependencies: {}
# [doc = " Iterator over the neighbors of a node."] # [doc = ""] # [doc = " Iterator element type is `NodeIndex<Ix>`."] # [doc = ""] # [doc = " Created with [`.neighbors()`][1], [`.neighbors_directed()`][2] or"] # [doc = " [`.neighbors_undirected()`][3]."] # [doc = ""] # [doc = " [1]: struct.Graph.html#method.neighbors"] # [doc = " [2]: struct.Graph.html#method.neighbors_directed"] # [doc = " [3]: struct.Graph.html#method.neighbors_undirected"] # [derive (Debug)] pub struct Neighbors < 'a , E : 'a , Ix : 'a = DefaultIx > { # [doc = " starting node to skip over"] skip_start : NodeIndex < Ix > , edges : & 'a [Edge < E , Ix >] , next : [EdgeIndex < Ix > ; 2] , }
};
}
