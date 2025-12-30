// Generated macro for has_path_connecting (function)
macro_rules! Depcrate_algohas_path_connecting {
() => {
// Module: crate::algo
// Provides: {"has_path_connecting"}
// Dependencies: {}
# [doc = " Check if there exists a path starting at `from` and reaching `to`."] # [doc = ""] # [doc = " If `from` and `to` are equal, this function returns true."] # [doc = ""] # [doc = " # Arguments:"] # [doc = " * `g`: an input graph."] # [doc = " * `from`: the first node of a desired path."] # [doc = " * `to`: the last node of a desired path."] # [doc = " * `space`: optional [`DfsSpace`]. If `space` is not `None`,"] # [doc = "   it is used instead of creating a new workspace for graph traversal."] # [doc = ""] # [doc = " # Returns"] # [doc = " * `true`: if there exists a path starting at `from` and reaching"] # [doc = "   `to` or `from` and `to` are equal."] # [doc = " * `false`: otherwise."] # [doc = ""] # [doc = " # Complexity"] # [doc = " * Time complexity: **O(|V| + |E|)**."] # [doc = " * Auxiliary space: **O(|V|)** or **O(1)** if `space` was provided."] # [doc = ""] # [doc = " where **|V|** is the number of nodes and **|E|** is the number of edges."] pub fn has_path_connecting < G > (g : G , from : G :: NodeId , to : G :: NodeId , space : Option < & mut DfsSpace < G :: NodeId , G :: Map > > ,) -> bool where G : IntoNeighbors + Visitable , { with_dfs (g , space , | dfs | { dfs . reset (g) ; dfs . move_to (from) ; dfs . iter (g) . any (| x | x == to) }) }
};
}
