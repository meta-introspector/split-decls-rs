// Generated macro for Neighbors (struct)
macro_rules! Depcrate_matrix_graphNeighbors {
() => {
// Module: crate::matrix_graph
// Provides: {"Neighbors"}
// Dependencies: {}
# [doc = " Iterator over the neighbors of a node."] # [doc = ""] # [doc = " Iterator element type is `NodeIndex<Ix>`."] # [doc = ""] # [doc = " Created with [`.neighbors()`][1], [`.neighbors_directed()`][2]."] # [doc = ""] # [doc = " [1]: struct.MatrixGraph.html#method.neighbors"] # [doc = " [2]: struct.MatrixGraph.html#method.neighbors_directed"] # [derive (Debug , Clone)] pub struct Neighbors < 'a , Ty : EdgeType , Null : 'a + Nullable , Ix > (Edges < 'a , Ty , Null , Ix >) ;
};
}
