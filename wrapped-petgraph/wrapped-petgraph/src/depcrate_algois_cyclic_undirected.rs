// Generated macro for is_cyclic_undirected (function)
macro_rules! Depcrate_algois_cyclic_undirected {
() => {
// Module: crate::algo
// Provides: {"is_cyclic_undirected"}
// Dependencies: {}
# [doc = " Return `true` if the input graph contains a cycle."] # [doc = ""] # [doc = " Always treats the input graph as if undirected."] # [doc = ""] # [doc = " # Arguments:"] # [doc = " `g`: an input graph that always treated as undirected."] # [doc = ""] # [doc = " # Returns"] # [doc = " `true`: if the input graph contains a cycle."] # [doc = " `false`: otherwise."] # [doc = ""] # [doc = " # Complexity"] # [doc = " * Time complexity: amortized **O(|E|)**."] # [doc = " * Auxiliary space: **O(|V|)**."] # [doc = ""] # [doc = " where **|V|** is the number of nodes and **|E|** is the number of edges."] pub fn is_cyclic_undirected < G > (g : G) -> bool where G : NodeIndexable + IntoEdgeReferences , { let mut edge_sets = UnionFind :: new (g . node_bound ()) ; for edge in g . edge_references () { let (a , b) = (edge . source () , edge . target ()) ; if ! edge_sets . union (g . to_index (a) , g . to_index (b)) { return true ; } } false }
};
}
