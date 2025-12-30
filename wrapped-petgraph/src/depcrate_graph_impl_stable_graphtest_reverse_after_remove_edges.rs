// Generated macro for test_reverse_after_remove_edges (function)
macro_rules! Depcrate_graph_impl_stable_graphtest_reverse_after_remove_edges {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"test_reverse_after_remove_edges"}
// Dependencies: {}
# [test] fn test_reverse_after_remove_edges () { let mut gr = StableGraph :: < _ , _ > :: default () ; let a = gr . add_node ("a") ; let b = gr . add_node ("b") ; let c = gr . add_node ("c") ; let e_ab = gr . add_edge (a , b , 0) ; let e_bc = gr . add_edge (b , c , 0) ; gr . remove_edge (e_ab) ; gr . remove_edge (e_bc) ; gr . check_free_lists () ; gr . reverse () ; gr . check_free_lists () ; }
};
}
