// Generated macro for extend_with_edges (function)
macro_rules! Depcrate_graph_impl_stable_graphextend_with_edges {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"extend_with_edges"}
// Dependencies: {}
# [test] fn extend_with_edges () { let mut gr = StableGraph :: < _ , _ > :: default () ; let a = gr . add_node ("a") ; let b = gr . add_node ("b") ; let c = gr . add_node ("c") ; let _d = gr . add_node ("d") ; gr . remove_node (a) ; gr . remove_node (b) ; gr . remove_node (c) ; gr . extend_with_edges (vec ! [(0 , 1 , ())]) ; assert_eq ! (gr . node_count () , 3) ; assert_eq ! (gr . edge_count () , 1) ; gr . check_free_lists () ; gr . extend_with_edges (vec ! [(5 , 1 , ())]) ; assert_eq ! (gr . node_count () , 4) ; assert_eq ! (gr . edge_count () , 2) ; gr . check_free_lists () ; }
};
}
