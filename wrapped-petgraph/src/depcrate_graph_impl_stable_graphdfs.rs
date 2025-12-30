// Generated macro for dfs (function)
macro_rules! Depcrate_graph_impl_stable_graphdfs {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"dfs"}
// Dependencies: {}
# [test] fn dfs () { use std :: println ; use crate :: visit :: Dfs ; let mut gr = StableGraph :: < _ , _ > :: with_capacity (0 , 0) ; let a = gr . add_node ("a") ; let b = gr . add_node ("b") ; let c = gr . add_node ("c") ; let d = gr . add_node ("d") ; gr . add_edge (a , b , 1) ; gr . add_edge (a , c , 2) ; gr . add_edge (b , c , 3) ; gr . add_edge (b , d , 4) ; gr . add_edge (c , d , 5) ; gr . add_edge (d , b , 6) ; gr . add_edge (c , b , 7) ; println ! ("{gr:?}") ; let mut dfs = Dfs :: new (& gr , a) ; while let Some (next) = dfs . next (& gr) { println ! ("dfs visit => {:?}, weight={:?}" , next , & gr [next]) ; } }
};
}
