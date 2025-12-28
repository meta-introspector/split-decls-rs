macro_rules! deps {
    () => {
        Graph!();
        Directed!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! complement {
    () => {
        deps!();
        # [doc = " \\[Generic\\] complement of the graph"] # [doc = ""] # [doc = " Computes the graph complement of the input Graph and stores it"] # [doc = " in the provided empty output Graph."] # [doc = ""] # [doc = " The function does not create self-loops."] # [doc = ""] # [doc = " Computes in **O(|V|^2*log(|V|))** time (average)."] # [doc = ""] # [doc = " Returns the complement."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use petgraph::Graph;"] # [doc = " use petgraph::operator::complement;"] # [doc = " use petgraph::prelude::*;"] # [doc = ""] # [doc = " let mut graph: Graph<(),(),Directed> = Graph::new();"] # [doc = " let a = graph.add_node(()); // node with no weight"] # [doc = " let b = graph.add_node(());"] # [doc = " let c = graph.add_node(());"] # [doc = " let d = graph.add_node(());"] # [doc = ""] # [doc = " graph.extend_with_edges(&["] # [doc = "     (a, b),"] # [doc = "     (b, c),"] # [doc = "     (c, d),"] # [doc = " ]);"] # [doc = " // a ----> b ----> c ----> d"] # [doc = ""] # [doc = " let mut output: Graph<(), (), Directed> = Graph::new();"] # [doc = ""] # [doc = " complement(&graph, &mut output, ());"] # [doc = ""] # [doc = " let mut expected_res: Graph<(), (), Directed> = Graph::new();"] # [doc = " let a = expected_res.add_node(());"] # [doc = " let b = expected_res.add_node(());"] # [doc = " let c = expected_res.add_node(());"] # [doc = " let d = expected_res.add_node(());"] # [doc = " expected_res.extend_with_edges(&["] # [doc = "     (a, c),"] # [doc = "     (a, d),"] # [doc = "     (b, a),"] # [doc = "     (b, d),"] # [doc = "     (c, a),"] # [doc = "     (c, b),"] # [doc = "     (d, a),"] # [doc = "     (d, b),"] # [doc = "     (d, c),"] # [doc = " ]);"] # [doc = ""] # [doc = " for x in graph.node_indices() {"] # [doc = "     for y in graph.node_indices() {"] # [doc = "         assert_eq!(output.contains_edge(x, y), expected_res.contains_edge(x, y));"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub fn complement < N , E , Ty , Ix > (input : & Graph < N , E , Ty , Ix > , output : & mut Graph < N , E , Ty , Ix > , weight : E ,) where Ty : EdgeType , Ix : IndexType , E : Clone , N : Clone , { for (_node , weight) in input . node_references () { output . add_node (weight . clone ()) ; } for x in input . node_indices () { for y in input . node_indices () { if x != y && ! input . contains_edge (x , y) { output . add_edge (x , y , weight . clone ()) ; } } } }
    };
}

complement!();