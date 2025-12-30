// Generated macro for dfs (function)
macro_rules! Depcrate_graph_vec_graph_testsdfs {
() => {
// Module: crate::graph::vec_graph::tests
// Provides: {"dfs"}
// Dependencies: {}
# [test] fn dfs () { let graph = create_graph () ; let dfs : Vec < _ > = graph :: depth_first_search (& graph , 0) . collect () ; assert_eq ! (dfs , vec ! [0 , 1 , 3 , 4 , 2]) ; let graph = create_graph_with_back_refs () ; let dfs : Vec < _ > = graph :: depth_first_search (& graph , 0) . collect () ; assert_eq ! (dfs , vec ! [0 , 1 , 3 , 4 , 2]) ; }
};
}
