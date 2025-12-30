// Generated macro for graph_from_str (macro)
macro_rules! Depcrate_dot_dot_parsergraph_from_str {
() => {
// Module: crate::dot::dot_parser
// Provides: {"graph_from_str"}
// Dependencies: {}
# [macro_export] # [doc = " Statically imports a Graph from a valid DOT/Graphviz [&str]."] macro_rules ! graph_from_str { ($ s : tt) => { $ crate :: dot :: dot_parser :: ParseFromDot :: from_dot_graph (dot_parser_macros :: from_dot_string ! ($ s)) } ; }
};
}
