macro_rules! deps {
    () => {
        NodeRef!();
        EdgeRef!();
        Graph!();
        Config!();
    };
}

macro_rules! Dot {
    () => {
        deps!();
        # [doc = " `Dot` implements output to graphviz .dot format for a graph."] # [doc = ""] # [doc = " Formatting and options are rather simple, this is mostly intended"] # [doc = " for debugging. Exact output may change."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use petgraph::Graph;"] # [doc = " use petgraph::dot::{Dot, Config};"] # [doc = ""] # [doc = " let mut graph = Graph::<_, ()>::new();"] # [doc = " graph.add_node(\"A\");"] # [doc = " graph.add_node(\"B\");"] # [doc = " graph.add_node(\"C\");"] # [doc = " graph.add_node(\"D\");"] # [doc = " graph.extend_with_edges(&["] # [doc = "     (0, 1), (0, 2), (0, 3),"] # [doc = "     (1, 2), (1, 3),"] # [doc = "     (2, 3),"] # [doc = " ]);"] # [doc = ""] # [doc = " println!(\"{:?}\", Dot::with_config(&graph, &[Config::EdgeNoLabel]));"] # [doc = ""] # [doc = " // In this case the output looks like this:"] # [doc = " //"] # [doc = " // digraph {"] # [doc = " //     0 [label=\"\\\"A\\\"\"]"] # [doc = " //     1 [label=\"\\\"B\\\"\"]"] # [doc = " //     2 [label=\"\\\"C\\\"\"]"] # [doc = " //     3 [label=\"\\\"D\\\"\"]"] # [doc = " //     0 -> 1 [ ]"] # [doc = " //     0 -> 2 [ ]"] # [doc = " //     0 -> 3 [ ]"] # [doc = " //     1 -> 2 [ ]"] # [doc = " //     1 -> 3 [ ]"] # [doc = " //     2 -> 3 [ ]"] # [doc = " // }"] # [doc = ""] # [doc = " // If you need multiple config options, just list them all in the slice."] # [doc = " ```"] pub struct Dot < 'a , G > where G : IntoEdgeReferences + IntoNodeReferences , { graph : G , get_edge_attributes : & 'a dyn Fn (G , G :: EdgeRef) -> String , get_node_attributes : & 'a dyn Fn (G , G :: NodeRef) -> String , config : Configs , }
    };
}

Dot!()