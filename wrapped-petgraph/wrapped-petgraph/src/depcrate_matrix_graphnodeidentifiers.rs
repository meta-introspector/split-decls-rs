// Generated macro for NodeIdentifiers (struct)
macro_rules! Depcrate_matrix_graphNodeIdentifiers {
() => {
// Module: crate::matrix_graph
// Provides: {"NodeIdentifiers"}
// Dependencies: {}
# [doc = " Iterator over the node identifiers of a graph."] # [doc = ""] # [doc = " Created from a call to [`.node_identifiers()`][1] on a [`MatrixGraph`][2]."] # [doc = ""] # [doc = " [1]: ../visit/trait.IntoNodeIdentifiers.html#tymethod.node_identifiers"] # [doc = " [2]: struct.MatrixGraph.html"] # [derive (Debug , Clone)] pub struct NodeIdentifiers < 'a , Ix , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , > { iter : IdIterator < 'a , S > , ix : PhantomData < Ix > , }
};
}
