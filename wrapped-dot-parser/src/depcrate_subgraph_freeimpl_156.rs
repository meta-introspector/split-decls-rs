// Generated macro for impl_156 (impl)
macro_rules! Depcrate_subgraph_freeimpl_156 {
() => {
// Module: crate::subgraph_free
// Provides: {"impl_156"}
// Dependencies: {}
impl < A > EdgeStmt < A > { fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> EdgeStmt < B > { EdgeStmt { from : self . from , next : self . next , attr : self . attr . map (| a | a . filter_map_attr (f)) , } } fn get_node_ids (& self) -> HashSet < NodeID > { let mut nexts = self . next . get_node_ids () ; nexts . insert (self . from . clone ()) ; nexts } }
};
}
