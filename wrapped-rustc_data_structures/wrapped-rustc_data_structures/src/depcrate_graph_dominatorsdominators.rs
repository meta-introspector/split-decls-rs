// Generated macro for dominators (function)
macro_rules! Depcrate_graph_dominatorsdominators {
() => {
// Module: crate::graph::dominators
// Provides: {"dominators"}
// Dependencies: {}
pub fn dominators < G : ControlFlowGraph > (g : & G) -> Dominators < G :: Node > { if is_small_path_graph (g) { Dominators { kind : Kind :: Path } } else { Dominators { kind : Kind :: General (dominators_impl (g)) } } }
};
}
