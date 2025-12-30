// Generated macro for Neighbors (struct)
macro_rules! Depcrate_graph_impl_stable_graphNeighbors {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"Neighbors"}
// Dependencies: {}
# [doc = " Iterator over the neighbors of a node."] # [doc = ""] # [doc = " Iterator element type is `NodeIndex`."] # [derive (Debug , Clone)] pub struct Neighbors < 'a , E : 'a , Ix : 'a = DefaultIx > { # [doc = " starting node to skip over"] skip_start : NodeIndex < Ix > , edges : & 'a [Edge < Option < E > , Ix >] , next : [EdgeIndex < Ix > ; 2] , }
};
}
