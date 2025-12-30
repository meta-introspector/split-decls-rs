// Generated macro for impl_1314 (impl)
macro_rules! Depcrate_matrix_graphimpl_1314 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1314"}
// Dependencies: {}
impl < Ty : EdgeType , Null : Nullable , Ix : IndexType > Iterator for Neighbors < '_ , Ty , Null , Ix > { type Item = NodeIndex < Ix > ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| (_ , b , _) | b) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
