// Generated macro for impl_175 (impl)
macro_rules! Depcrate_graph_linked_graphimpl_175 {
() => {
// Module: crate::graph::linked_graph
// Provides: {"impl_175"}
// Dependencies: {}
impl < 'g , N : Debug , E : Debug > Iterator for DepthFirstTraversal < 'g , N , E > { type Item = NodeIndex ; fn next (& mut self) -> Option < NodeIndex > { let next = self . stack . pop () ; if let Some (idx) = next { for (_ , edge) in self . graph . adjacent_edges (idx , self . direction) { let target = edge . source_or_target (self . direction) ; self . visit (target) ; } } next } fn size_hint (& self) -> (usize , Option < usize >) { let remaining = self . graph . len_nodes () - self . visited . count () ; (remaining , Some (remaining)) } }
};
}
