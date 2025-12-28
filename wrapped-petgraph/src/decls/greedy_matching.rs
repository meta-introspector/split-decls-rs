macro_rules! deps {
    () => {
        Time!();
        Matching!();
    };
}

macro_rules! greedy_matching {
    () => {
        deps!();
        # [doc = " Compute a [*matching*](https://en.wikipedia.org/wiki/Matching_(graph_theory)) using a"] # [doc = " greedy heuristic."] # [doc = ""] # [doc = " The input graph is treated as if undirected. The underlying heuristic is"] # [doc = " unspecified, but is guaranteed to be bounded by **O(|V| + |E|)**. No"] # [doc = " guarantees about the output are given other than that it is a valid"] # [doc = " matching."] # [doc = ""] # [doc = " If you require a maximum matching, use [`maximum_matching`][1] function"] # [doc = " instead."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `graph`: an undirected graph."] # [doc = ""] # [doc = " # Returns"] # [doc = " * [`struct@Matching`] calculated using greedy heuristic."] # [doc = ""] # [doc = " # Complexity"] # [doc = " * Time complexity: **O(|V| + |E|)**."] # [doc = " * Auxiliary space: **O(|V|)**."] # [doc = ""] # [doc = " where **|V|** is the number of nodes and **|E|** is the number of edges."] # [doc = ""] # [doc = " [1]: fn.maximum_matching.html"] pub fn greedy_matching < G > (graph : G) -> Matching < G > where G : Visitable + IntoNodeIdentifiers + NodeIndexable + IntoNeighbors , G :: NodeId : Eq + Hash , G :: EdgeId : Eq + Hash , { let (mates , n_edges) = greedy_matching_inner (& graph) ; Matching :: new (graph , mates , n_edges) }
    };
}

greedy_matching!()