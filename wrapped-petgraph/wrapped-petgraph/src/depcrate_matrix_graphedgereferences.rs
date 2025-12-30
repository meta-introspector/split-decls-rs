// Generated macro for EdgeReferences (struct)
macro_rules! Depcrate_matrix_graphEdgeReferences {
() => {
// Module: crate::matrix_graph
// Provides: {"EdgeReferences"}
// Dependencies: {}
# [doc = " Iterator over all edges of a graph."] # [doc = ""] # [doc = " Created from a call to [`.edge_references()`][1] on a [`MatrixGraph`][2]."] # [doc = ""] # [doc = " [1]: ../visit/trait.IntoEdgeReferences.html#tymethod.edge_references"] # [doc = " [2]: struct.MatrixGraph.html"] # [derive (Debug , Clone)] pub struct EdgeReferences < 'a , Ty : EdgeType , Null : 'a + Nullable , Ix > { row : usize , column : usize , node_adjacencies : & 'a [Null] , node_capacity : usize , ty : PhantomData < Ty > , ix : PhantomData < Ix > , }
};
}
