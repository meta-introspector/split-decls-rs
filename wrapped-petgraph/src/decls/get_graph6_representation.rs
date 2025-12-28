macro_rules! get_graph6_representation {
    () => {
        # [doc = " Converts a graph that implements GetAdjacencyMatrix and IntoNodeIdentifers"] # [doc = " into a graph6 format string."] pub fn get_graph6_representation < G > (graph : G) -> String where G : GetAdjacencyMatrix + IntoNodeIdentifiers , { let (graph_order , mut upper_diagonal_as_bits) = get_adj_matrix_upper_diagonal_as_bits (graph) ; let mut graph_order_as_bits = get_graph_order_as_bits (graph_order) ; let mut graph_as_bits = vec ! [] ; graph_as_bits . append (& mut graph_order_as_bits) ; graph_as_bits . append (& mut upper_diagonal_as_bits) ; bits_to_ascii (graph_as_bits) }
    };
}

get_graph6_representation!()