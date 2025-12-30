// Generated macro for stable_graph (function)
macro_rules! Depcrate_graph_impl_stable_graphstable_graph {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"stable_graph"}
// Dependencies: {}
# [test] fn stable_graph () { use std :: println ; let mut gr = StableGraph :: < _ , _ > :: with_capacity (0 , 0) ; let a = gr . add_node (0) ; let b = gr . add_node (1) ; let c = gr . add_node (2) ; let _ed = gr . add_edge (a , b , 1) ; println ! ("{gr:?}") ; gr . remove_node (b) ; println ! ("{gr:?}") ; let d = gr . add_node (3) ; println ! ("{gr:?}") ; gr . check_free_lists () ; gr . remove_node (a) ; gr . check_free_lists () ; gr . remove_node (c) ; gr . check_free_lists () ; println ! ("{gr:?}") ; gr . add_edge (d , d , 2) ; println ! ("{gr:?}") ; let e = gr . add_node (4) ; gr . add_edge (d , e , 3) ; println ! ("{gr:?}") ; for neigh in gr . neighbors (d) { println ! ("edge {d:?} -> {neigh:?}") ; } gr . check_free_lists () ; }
};
}
