// Generated macro for WalkNeighbors (struct)
macro_rules! Depcrate_graph_impl_stable_graphWalkNeighbors {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"WalkNeighbors"}
// Dependencies: {}
# [doc = " A “walker” object that can be used to step through the edge list of a node."] # [doc = ""] # [doc = " See [*.detach()*](struct.Neighbors.html#method.detach) for more information."] # [doc = ""] # [doc = " The walker does not borrow from the graph, so it lets you step through"] # [doc = " neighbors or incident edges while also mutating graph weights, as"] # [doc = " in the following example:"] # [doc = ""] # [doc = " ```"] # [doc = " use petgraph::visit::Dfs;"] # [doc = " use petgraph::Incoming;"] # [doc = " use petgraph::stable_graph::StableGraph;"] # [doc = ""] # [doc = " let mut gr = StableGraph::new();"] # [doc = " let a = gr.add_node(0.);"] # [doc = " let b = gr.add_node(0.);"] # [doc = " let c = gr.add_node(0.);"] # [doc = " gr.add_edge(a, b, 3.);"] # [doc = " gr.add_edge(b, c, 2.);"] # [doc = " gr.add_edge(c, b, 1.);"] # [doc = ""] # [doc = " // step through the graph and sum incoming edges into the node weight"] # [doc = " let mut dfs = Dfs::new(&gr, a);"] # [doc = " while let Some(node) = dfs.next(&gr) {"] # [doc = "     // use a detached neighbors walker"] # [doc = "     let mut edges = gr.neighbors_directed(node, Incoming).detach();"] # [doc = "     while let Some(edge) = edges.next_edge(&gr) {"] # [doc = "         gr[node] += gr[edge];"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " // check the result"] # [doc = " assert_eq!(gr[a], 0.);"] # [doc = " assert_eq!(gr[b], 4.);"] # [doc = " assert_eq!(gr[c], 2.);"] # [doc = " ```"] pub struct WalkNeighbors < Ix > { inner : super :: WalkNeighbors < Ix > , }
};
}
