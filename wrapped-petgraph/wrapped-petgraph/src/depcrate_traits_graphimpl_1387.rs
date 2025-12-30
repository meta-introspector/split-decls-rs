// Generated macro for impl_1387 (impl)
macro_rules! Depcrate_traits_graphimpl_1387 {
() => {
// Module: crate::traits_graph
// Provides: {"impl_1387"}
// Dependencies: {}
# [cfg (feature = "stable_graph")] # [doc = " The adjacency matrix for **Graph** is a bitmap that's computed by"] # [doc = " `.adjacency_matrix()`."] impl < N , E , Ty , Ix > GetAdjacencyMatrix for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type AdjMatrix = FixedBitSet ; fn adjacency_matrix (& self) -> FixedBitSet { let n = self . node_bound () ; let mut matrix = FixedBitSet :: with_capacity (n * n) ; for edge in self . edge_references () { let i = edge . source () . index () * n + edge . target () . index () ; matrix . put (i) ; if ! self . is_directed () { let j = edge . source () . index () + n * edge . target () . index () ; matrix . put (j) ; } } matrix } fn is_adjacent (& self , matrix : & FixedBitSet , a : NodeIndex < Ix > , b : NodeIndex < Ix >) -> bool { let n = self . node_count () ; let index = n * a . index () + b . index () ; matrix . contains (index) } }
};
}
