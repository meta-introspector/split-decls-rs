// Generated macro for Dfs (struct)
macro_rules! Depcrate_visit_traversalDfs {
() => {
// Module: crate::visit::traversal
// Provides: {"Dfs"}
// Dependencies: {}
# [doc = " Visit nodes of a graph in a depth-first-search (DFS) emitting nodes in"] # [doc = " preorder (when they are first discovered)."] # [doc = ""] # [doc = " The traversal starts at a given node and only traverses nodes reachable"] # [doc = " from it."] # [doc = ""] # [doc = " `Dfs` is not recursive."] # [doc = ""] # [doc = " `Dfs` does not itself borrow the graph, and because of this you can run"] # [doc = " a traversal over a graph while still retaining mutable access to it, if you"] # [doc = " use it like the following example:"] # [doc = ""] # [doc = " ```"] # [doc = " use petgraph::Graph;"] # [doc = " use petgraph::visit::Dfs;"] # [doc = ""] # [doc = " let mut graph = Graph::<_,()>::new();"] # [doc = " let a = graph.add_node(0);"] # [doc = ""] # [doc = " let mut dfs = Dfs::new(&graph, a);"] # [doc = " while let Some(nx) = dfs.next(&graph) {"] # [doc = "     // we can access `graph` mutably here still"] # [doc = "     graph[nx] += 1;"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(graph[a], 1);"] # [doc = " ```"] # [doc = ""] # [doc = " **Note:** The algorithm may not behave correctly if nodes are removed"] # [doc = " during iteration. It may not necessarily visit added nodes or edges."] # [derive (Clone , Debug)] pub struct Dfs < N , VM > { # [doc = " The stack of nodes to visit"] pub stack : Vec < N > , # [doc = " The map of discovered nodes"] pub discovered : VM , }
};
}
