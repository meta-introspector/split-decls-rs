// Generated macro for Edges (struct)
macro_rules! Depcrate_matrix_graphEdges {
() => {
// Module: crate::matrix_graph
// Provides: {"Edges"}
// Dependencies: {}
# [doc = " Iterator over the edges of from or to a node"] # [doc = ""] # [doc = " Created with [`.edges()`][1], [`.edges_directed()`][2]."] # [doc = ""] # [doc = " [1]: struct.MatrixGraph.html#method.edges"] # [doc = " [2]: struct.MatrixGraph.html#method.edges_directed"] # [derive (Debug , Clone)] pub struct Edges < 'a , Ty : EdgeType , Null : 'a + Nullable , Ix > { iter_direction : NeighborIterDirection , node_adjacencies : & 'a [Null] , node_capacity : usize , row : usize , column : usize , ty : PhantomData < Ty > , ix : PhantomData < Ix > , }
};
}
