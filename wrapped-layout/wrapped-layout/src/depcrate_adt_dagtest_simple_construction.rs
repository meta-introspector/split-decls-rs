// Generated macro for test_simple_construction (function)
macro_rules! Depcrate_adt_dagtest_simple_construction {
() => {
// Module: crate::adt::dag
// Provides: {"test_simple_construction"}
// Dependencies: {}
# [test] fn test_simple_construction () { let mut g = DAG :: new () ; let h0 = g . new_node () ; g . verify () ; let h1 = g . new_node () ; let h2 = g . new_node () ; let h3 = g . new_node () ; let h4 = g . new_node () ; assert_ne ! (h0 , h1) ; assert_ne ! (h1 , h2) ; g . add_edge (h0 , h1) ; g . add_edge (h1 , h2) ; g . add_edge (h0 , h2) ; g . add_edge (h2 , h3) ; g . add_edge (h3 , h4) ; g . verify () ; let order = g . topological_sort () ; let levels = g . compute_levels (& order) ; assert_eq ! (order . len () , g . len ()) ; assert_eq ! (levels . len () , g . len ()) ; for i in 0 .. g . len () { println ! ("{}) node {},  level {}" , i , order [i] . idx , levels [i]) ; } }
};
}
