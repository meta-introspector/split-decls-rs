// Generated macro for example_graph (function)
macro_rules! Depcrate_doctest_helpersexample_graph {
() => {
// Module: crate::doctest_helpers
// Provides: {"example_graph"}
// Dependencies: {}
pub fn example_graph () -> Graph { let mut g = Graph :: default () ; let mut g_ref = g . into_partial_ref_mut () ; g_ref . part_mut (Colors) . extend (& [0 , 1 , 0]) ; g_ref . part_mut (Weights) . extend (& [0.25 , 0.5 , 0.75]) ; g_ref . part_mut (Neighbors) . push (vec ! [1 , 2]) ; g_ref . part_mut (Neighbors) . push (vec ! [0 , 2]) ; g_ref . part_mut (Neighbors) . push (vec ! [0 , 1]) ; g }
};
}
