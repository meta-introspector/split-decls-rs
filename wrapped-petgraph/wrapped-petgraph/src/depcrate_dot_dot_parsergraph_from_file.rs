// Generated macro for graph_from_file (macro)
macro_rules! Depcrate_dot_dot_parsergraph_from_file {
() => {
// Module: crate::dot::dot_parser
// Provides: {"graph_from_file"}
// Dependencies: {}
# [macro_export] # [doc = " Statically imports a Graph from a DOT/Graphviz file. The macro expects the file path as argument."] # [doc = ""] # [doc = " Notice that, since the graph is imported *statically*, the file must exist at compile time, but"] # [doc = " can be removed at runtime."] macro_rules ! graph_from_file { ($ s : tt) => { $ crate :: dot :: dot_parser :: ParseFromDot :: from_dot_graph (dot_parser_macros :: from_dot_file ! ($ s)) } ; }
};
}
