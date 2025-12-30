// Generated macro for impl_530 (impl)
macro_rules! Depcrate_algo_matchingimpl_530 {
() => {
// Module: crate::algo::matching
// Provides: {"impl_530"}
// Dependencies: {}
impl < G > Matching < G > where G : NodeCount , { # [doc = " Returns `true` if the matching is perfect."] # [doc = ""] # [doc = " A matching is"] # [doc = " [*perfect*](https://en.wikipedia.org/wiki/Matching_(graph_theory)#Definitions)"] # [doc = " if every node in the graph is incident to an edge from the matching."] pub fn is_perfect (& self) -> bool { let n_nodes = self . graph . node_count () ; n_nodes % 2 == 0 && self . n_edges == n_nodes / 2 } }
};
}
