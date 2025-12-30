// Generated macro for impl_779 (impl)
macro_rules! Depcrate_csrimpl_779 {
() => {
// Module: crate::csr
// Provides: {"impl_779"}
// Dependencies: {}
# [doc = " The adjacency matrix for **Csr** is a bitmap that's computed by"] # [doc = " `.adjacency_matrix()`."] impl < N , E , Ty , Ix > GetAdjacencyMatrix for & Csr < N , E , Ty , Ix > where Ix : IndexType , Ty : EdgeType , { type AdjMatrix = FixedBitSet ; fn adjacency_matrix (& self) -> FixedBitSet { let n = self . node_count () ; let mut matrix = FixedBitSet :: with_capacity (n * n) ; for edge in self . edge_references () { let i = n * edge . source () . index () + edge . target () . index () ; matrix . put (i) ; if ! self . is_directed () { let j = edge . source () . index () + n * edge . target () . index () ; matrix . put (j) ; } } matrix } fn is_adjacent (& self , matrix : & FixedBitSet , a : NodeIndex < Ix > , b : NodeIndex < Ix >) -> bool { let n = self . node_count () ; let index = n * a . index () + b . index () ; matrix . contains (index) } }
};
}
