// Generated macro for test_rank_api (function)
macro_rules! Depcrate_adt_dagtest_rank_api {
() => {
// Module: crate::adt::dag
// Provides: {"test_rank_api"}
// Dependencies: {}
# [test] fn test_rank_api () { let mut g = DAG :: new () ; let h0 = g . new_node () ; let h1 = g . new_node () ; let h2 = g . new_node () ; g . add_edge (h0 , h1) ; g . add_edge (h1 , h2) ; g . recompute_node_ranks () ; g . verify () ; assert_eq ! (g . level (h0) , 0) ; assert_eq ! (g . level (h1) , 1) ; assert_eq ! (g . level (h2) , 2) ; let r1 = g . remove_edge (h0 , h1) ; let r2 = g . remove_edge (h0 , h1) ; assert ! (r1) ; assert ! (! r2) ; }
};
}
