// Generated macro for DiGraphMap (type)
macro_rules! Depcrate_graphmapDiGraphMap {
() => {
// Module: crate::graphmap
// Provides: {"DiGraphMap"}
// Dependencies: {}
# [doc = " A `GraphMap` with directed edges."] # [doc = ""] # [doc = " For example, an edge from *1* to *2* is distinct from an edge from *2* to"] # [doc = " *1*."] pub type DiGraphMap < N , E , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState > = GraphMap < N , E , Directed , S > ;
};
}
