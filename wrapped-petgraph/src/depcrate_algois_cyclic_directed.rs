// Generated macro for is_cyclic_directed (function)
macro_rules! Depcrate_algois_cyclic_directed {
() => {
// Module: crate::algo
// Provides: {"is_cyclic_directed"}
// Dependencies: {}
# [doc = " Return `true` if the input directed graph contains a cycle."] # [doc = ""] # [doc = " This implementation is recursive; use [`toposort`] if an alternative is needed."] # [doc = ""] # [doc = " # Arguments:"] # [doc = " `g`: a directed graph."] # [doc = ""] # [doc = " # Returns"] # [doc = " `true`: if the input graph contains a cycle."] # [doc = " `false`: otherwise."] # [doc = ""] # [doc = " # Complexity"] # [doc = " * Time complexity: **O(|V| + |E|)**."] # [doc = " * Auxiliary space: **O(|V|)**."] # [doc = ""] # [doc = " where **|V|** is the number of nodes and **|E|** is the number of edges."] pub fn is_cyclic_directed < G > (g : G) -> bool where G : IntoNodeIdentifiers + IntoNeighbors + Visitable , { use crate :: visit :: { DfsEvent , depth_first_search } ; depth_first_search (g , g . node_identifiers () , | event | match event { DfsEvent :: BackEdge (_ , _) => Err (()) , _ => Ok (()) , }) . is_err () }
};
}
