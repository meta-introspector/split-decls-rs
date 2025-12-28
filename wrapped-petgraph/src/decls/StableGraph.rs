macro_rules! deps {
    () => {
        Directed!();
        EdgeIndex!();
        NodeIndex!();
        Dfs!();
        Graph!();
        DefaultIx!();
        Edge!();
    };
}

macro_rules! StableGraph {
    () => {
        deps!();
        # [doc = " `StableGraph<N, E, Ty, Ix>` is a graph datastructure using an adjacency"] # [doc = " list representation."] # [doc = ""] # [doc = " The graph **does not invalidate** any unrelated node or edge indices when"] # [doc = " items are removed."] # [doc = ""] # [doc = " `StableGraph` is parameterized over:"] # [doc = ""] # [doc = " - Associated data `N` for nodes and `E` for edges, also called *weights*."] # [doc = "   The associated data can be of arbitrary type."] # [doc = " - Edge type `Ty` that determines whether the graph edges are directed or undirected."] # [doc = " - Index type `Ix`, which determines the maximum size of the graph."] # [doc = ""] # [doc = " The graph uses **O(|V| + |E|)** space where V is the set of nodes and E is the"] # [doc = " set of edges, and allows fast node and edge insert and efficient graph search."] # [doc = ""] # [doc = " It implements **O(e')** edge lookup and edge and node removals, where **e'**"] # [doc = " is some local measure of edge count."] # [doc = ""] # [doc = " - Nodes and edges are each numbered in an interval from *0* to some number"] # [doc = "   *m*, but *not all* indices in the range are valid, since gaps are formed"] # [doc = "   by deletions."] # [doc = ""] # [doc = " - You can select graph index integer type after the size of the graph. A smaller"] # [doc = "   size may have better performance."] # [doc = ""] # [doc = " - Using indices allows mutation while traversing the graph, see `Dfs`."] # [doc = ""] # [doc = " - The `StableGraph` is a regular rust collection and is `Send` and `Sync`"] # [doc = "   (as long as associated data `N` and `E` are)."] # [doc = ""] # [doc = " - Indices don't allow as much compile time checking as references."] # [doc = ""] # [doc = " Depends on crate feature `stable_graph` (default). *Stable Graph is still"] # [doc = " missing a few methods compared to Graph. You can contribute to help it"] # [doc = " achieve parity.*"] pub struct StableGraph < N , E , Ty = Directed , Ix = DefaultIx > { g : Graph < Option < N > , Option < E > , Ty , Ix > , node_count : usize , edge_count : usize , free_node : NodeIndex < Ix > , free_edge : EdgeIndex < Ix > , }
    };
}

StableGraph!();