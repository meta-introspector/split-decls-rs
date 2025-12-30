// Generated macro for NodeReferences (struct)
macro_rules! Depcrate_matrix_graphNodeReferences {
() => {
// Module: crate::matrix_graph
// Provides: {"NodeReferences"}
// Dependencies: {}
# [doc = " Iterator over all nodes of a graph."] # [doc = ""] # [doc = " Created from a call to [`.node_references()`][1] on a [`MatrixGraph`][2]."] # [doc = ""] # [doc = " [1]: ../visit/trait.IntoNodeReferences.html#tymethod.node_references"] # [doc = " [2]: struct.MatrixGraph.html"] # [derive (Debug , Clone)] pub struct NodeReferences < 'a , N : 'a , Ix , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , > { nodes : & 'a IdStorage < N , S > , iter : IdIterator < 'a , S > , ix : PhantomData < Ix > , }
};
}
