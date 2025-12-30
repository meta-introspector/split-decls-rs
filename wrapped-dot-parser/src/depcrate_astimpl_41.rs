// Generated macro for impl_41 (impl)
macro_rules! Depcrate_astimpl_41 {
() => {
// Module: crate::ast
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a , A > StmtList < A > { fn filter_map_attr < B > (self , f : & 'a dyn Fn (A) -> Option < B >) -> StmtList < B > { self . stmts . into_iter () . map (| stmt | stmt . filter_map_attr (f)) . collect () } fn get_node_ids (& self) -> HashSet < NodeID > { let mut hs = HashSet :: new () ; for stmt in self { hs = hs . union (& stmt . get_node_ids ()) . cloned () . collect () ; } hs } }
};
}
