// Generated macro for Matching (struct)
macro_rules! Depcrate_algo_matchingMatching {
() => {
// Module: crate::algo::matching
// Provides: {"Matching"}
// Dependencies: {}
# [doc = " Computed"] # [doc = " [*matching*](https://en.wikipedia.org/wiki/Matching_(graph_theory)#Definitions)"] # [doc = " of the graph."] pub struct Matching < G : GraphBase > { graph : G , mate : Vec < Option < G :: NodeId > > , n_edges : usize , }
};
}
