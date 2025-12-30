// Generated macro for impl_312 (impl)
macro_rules! Depcrate_acyclicimpl_312 {
() => {
// Module: crate::acyclic
// Provides: {"impl_312"}
// Dependencies: {}
impl < G : Visitable + GetAdjacencyMatrix > GetAdjacencyMatrix for Acyclic < G > { type AdjMatrix = G :: AdjMatrix ; fn adjacency_matrix (& self) -> Self :: AdjMatrix { self . inner () . adjacency_matrix () } fn is_adjacent (& self , matrix : & Self :: AdjMatrix , a : Self :: NodeId , b : Self :: NodeId) -> bool { self . inner () . is_adjacent (matrix , a , b) } }
};
}
