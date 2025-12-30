// Generated macro for UnMatrix (type)
macro_rules! Depcrate_matrix_graphUnMatrix {
() => {
// Module: crate::matrix_graph
// Provides: {"UnMatrix"}
// Dependencies: {}
# [doc = " A `MatrixGraph` with undirected edges."] pub type UnMatrix < N , E , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , Null = Option < E > , Ix = DefaultIx , > = MatrixGraph < N , E , S , Undirected , Null , Ix > ;
};
}
