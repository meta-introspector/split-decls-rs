// Generated macro for dump_graph (function)
macro_rules! Depcrate_assert_dep_graphdump_graph {
() => {
// Module: crate::assert_dep_graph
// Provides: {"dump_graph"}
// Dependencies: {}
fn dump_graph (query : & DepGraphQuery) { let path : String = env :: var ("RUST_DEP_GRAPH") . unwrap_or_else (| _ | "dep_graph" . to_string ()) ; let nodes = match env :: var ("RUST_DEP_GRAPH_FILTER") { Ok (string) => { let edge_filter = EdgeFilter :: new (& string) . unwrap_or_else (| e | bug ! ("invalid filter: {}" , e)) ; let sources = node_set (query , & edge_filter . source) ; let targets = node_set (query , & edge_filter . target) ; filter_nodes (query , & sources , & targets) } Err (_) => query . nodes () . into_iter () . map (| n | n . kind) . collect () , } ; let edges = filter_edges (query , & nodes) ; { let txt_path = format ! ("{path}.txt") ; let mut file = File :: create_buffered (& txt_path) . unwrap () ; for (source , target) in & edges { write ! (file , "{source:?} -> {target:?}\n") . unwrap () ; } } { let dot_path = format ! ("{path}.dot") ; let mut v = Vec :: new () ; dot :: render (& GraphvizDepGraph (nodes , edges) , & mut v) . unwrap () ; fs :: write (dot_path , v) . unwrap () ; } }
};
}
