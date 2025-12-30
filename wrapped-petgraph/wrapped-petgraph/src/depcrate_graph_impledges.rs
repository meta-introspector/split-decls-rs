// Generated macro for Edges (struct)
macro_rules! Depcrate_graph_implEdges {
() => {
// Module: crate::graph_impl
// Provides: {"Edges"}
// Dependencies: {}
# [doc = " Iterator over the edges of from or to a node"] # [derive (Debug)] pub struct Edges < 'a , E : 'a , Ty , Ix : 'a = DefaultIx > where Ty : EdgeType , Ix : IndexType , { # [doc = " starting node to skip over"] skip_start : NodeIndex < Ix > , edges : & 'a [Edge < E , Ix >] , # [doc = " Next edge to visit."] next : [EdgeIndex < Ix > ; 2] , # [doc = " For directed graphs: the direction to iterate in"] # [doc = " For undirected graphs: the direction of edges"] direction : Direction , ty : PhantomData < Ty > , }
};
}
