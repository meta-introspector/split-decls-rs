// Generated macro for impl_159 (impl)
macro_rules! Depcrate_subgraph_freeimpl_159 {
() => {
// Module: crate::subgraph_free
// Provides: {"impl_159"}
// Dependencies: {}
impl EdgeRHS { fn get_node_ids (& self) -> HashSet < NodeID > { let mut nexts : HashSet < NodeID > = self . next . as_ref () . map (| n | n . get_node_ids ()) . unwrap_or_default () ; nexts . insert (self . to . clone ()) ; nexts } }
};
}
