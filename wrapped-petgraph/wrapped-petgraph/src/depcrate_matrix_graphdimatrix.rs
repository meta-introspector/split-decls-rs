// Generated macro for DiMatrix (type)
macro_rules! Depcrate_matrix_graphDiMatrix {
() => {
// Module: crate::matrix_graph
// Provides: {"DiMatrix"}
// Dependencies: {}
# [doc = " A `MatrixGraph` with directed edges."] pub type DiMatrix < N , E , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , Null = Option < E > , Ix = DefaultIx , > = MatrixGraph < N , E , S , Directed , Null , Ix > ;
};
}
