// Generated macro for Generation (type)
macro_rules! Depcrate_graphGeneration {
() => {
// Module: crate::graph
// Provides: {"Generation"}
// Dependencies: {}
# [doc = " The generation away from the HEAD of graph, useful to limit algorithms by topological depth as well."] # [doc = ""] # [doc = " 0 would mean the starting point of the hierarchy, and 1 their parents."] # [doc = " This number is only available natively if there is a commit-graph."] pub type Generation = u32 ;
};
}
