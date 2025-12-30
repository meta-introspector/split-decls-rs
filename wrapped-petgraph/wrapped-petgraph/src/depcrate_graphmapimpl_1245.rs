// Generated macro for impl_1245 (impl)
macro_rules! Depcrate_graphmapimpl_1245 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1245"}
// Dependencies: {}
# [doc = " The `GraphMap` keeps an adjacency matrix internally."] impl < N , E , Ty , S > visit :: GetAdjacencyMatrix for GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { type AdjMatrix = () ; # [inline] fn adjacency_matrix (& self) { } # [inline] fn is_adjacent (& self , _ : & () , a : N , b : N) -> bool { self . contains_edge (a , b) } }
};
}
