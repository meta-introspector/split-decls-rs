macro_rules! deps {
    () => {
        Graph!();
        Directed!();
        Build!();
    };
}

macro_rules! Acyclic {
    () => {
        deps!();
        # [doc = " A directed acyclic graph."] # [doc = ""] # [doc = " Wrap directed acyclic graphs and expose an API that ensures the invariant"] # [doc = " is maintained, i.e. no cycles can be created. This uses a topological order"] # [doc = " that is dynamically updated when edges are added. In the worst case, the"] # [doc = " runtime may be linear in the number of vertices, but it has been shown to"] # [doc = " be fast in practice, particularly on sparse graphs (Pierce and Kelly, 2004)."] # [doc = ""] # [doc = " To be modifiable (and hence to be useful), the graphs of generic type `G`"] # [doc = " should implement the [`Build`] trait. Good candidates for `G` are thus"] # [doc = " [`crate::graph::DiGraph`] and [`crate::stable_graph::StableDiGraph`]."] # [doc = ""] # [doc = " ## Algorithm"] # [doc = " This implements the PK algorithm for dynamic topological sort described in"] # [doc = " \"A Dynamic Topological Sort Algorithm for Directed Acyclic Graphs\" by"] # [doc = " D. Pierce and P. Kelly, JEA, 2004. It maintains a topological order of the"] # [doc = " nodes that can be efficiently updated when edges are added. Achieves a good"] # [doc = " balance between simplicity and performance in practice, see the paper for"] # [doc = " discussions of the running time."] # [doc = ""] # [doc = " ## Graph traits"] # [doc = " All graph traits are delegated to the inner graph, with the exception of"] # [doc = " the graph construction trait [`Build`]. The wrapped graph can thus only"] # [doc = " be modified through the wrapped API that ensures no cycles are created."] # [doc = ""] # [doc = " ## Behaviour on cycles"] # [doc = " By design, edge additions to this datatype may fail. It is recommended to"] # [doc = " prefer the dedicated [`Acyclic::try_add_edge`] and"] # [doc = " [`Acyclic::try_update_edge`] methods whenever possible. The"] # [doc = " [`Build::update_edge`] methods will panic if it is attempted to add an edge"] # [doc = " that would create a cycle. The [`Build::add_edge`] on the other hand method"] # [doc = " will return `None` if the edge cannot be added (either it already exists on"] # [doc = " a graph type that does not support it or would create a cycle)."] # [derive (Clone , Debug)] pub struct Acyclic < G : Visitable > { # [doc = " The underlying graph, accessible through the `inner` method."] graph : G , # [doc = " The current topological order of the nodes."] order_map : OrderMap < G :: NodeId > , # [doc = " Helper map for DFS tracking discovered nodes."] discovered : RefCell < FixedBitSet > , # [doc = " Helper map for DFS tracking finished nodes."] finished : RefCell < FixedBitSet > , }
    };
}

Acyclic!();