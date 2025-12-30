// Generated macro for ParseFromDot (trait)
macro_rules! Depcrate_dot_dot_parserParseFromDot {
() => {
// Module: crate::dot::dot_parser
// Provides: {"ParseFromDot"}
// Dependencies: {}
# [doc = " This trait extends [Create] with a method to parse a graph from a dot string."] pub trait ParseFromDot < 'a > : Create < EdgeWeight = DotAttrList < 'a > , NodeWeight = DotNodeWeight < 'a > > { # [doc = " Convert a DOT/Graphviz graph (represented as an [DotGraph]) into a petgraph's graph."] fn from_dot_graph (dot_graph : DotGraph < (& 'a str , & 'a str) >) -> Self { let dot_graph : CGraph < (& 'a str , & 'a str) > = dot_graph . into () ; let node_number = dot_graph . nodes . set . len () ; let edge_number = dot_graph . edges . set . len () ; let mut graph = Self :: with_capacity (node_number , edge_number) ; let mut node_indices = std :: collections :: HashMap :: new () ; for node in dot_graph . nodes . set { let ni = graph . add_node (node . 1) ; node_indices . insert (node . 0 , ni) ; } for edge in dot_graph . edges . set { let from_ni = node_indices . get (& edge . from) . unwrap () ; let to_ni = node_indices . get (& edge . to) . unwrap () ; graph . add_edge (* from_ni , * to_ni , edge . attr) ; } graph } # [doc = " Attempt to parse a DOT/Graphviz string into a graph. Fail if the string is not a"] # [doc = " well-formed DOT/Graphviz string."] fn try_from (s : & 'a str) -> Result < Self , DotParsingError > { let ast = DotGraph :: try_from (s) ? ; let petgraph = Self :: from_dot_graph (ast) ; Ok (petgraph) } }
};
}
