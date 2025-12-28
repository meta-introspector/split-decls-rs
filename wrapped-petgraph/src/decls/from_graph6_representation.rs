macro_rules! deps {
    () => {
        IndexType!();
    };
}

macro_rules! from_graph6_representation {
    () => {
        deps!();
        # [doc = " Converts a graph6 format string into data can be used to construct an undirected graph."] # [doc = " Returns a tuple containing the graph order and its edges."] pub fn from_graph6_representation < Ix > (graph6_representation : String) -> (usize , Vec < (Ix , Ix) >) where Ix : IndexType , { let (order_bytes , adj_matrix_bytes) = get_order_bytes_and_adj_matrix_bytes (graph6_representation) ; let order_bits = bytes_vector_to_bits_vector (order_bytes) ; let adj_matrix_bits = bytes_vector_to_bits_vector (adj_matrix_bytes) ; let graph_order = get_bits_as_decimal (order_bits) ; let edges = get_edges (graph_order , adj_matrix_bits) ; (graph_order , edges) }
    };
}

from_graph6_representation!();