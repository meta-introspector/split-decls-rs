// Generated macro for test_find_state_2 (function)
macro_rules! Depcrate_graph_scc_teststest_find_state_2 {
() => {
// Module: crate::graph::scc::tests
// Provides: {"test_find_state_2"}
// Dependencies: {}
# [test] fn test_find_state_2 () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 4) , (1 , 2) , (1 , 3) , (2 , 1) , (3 , 0) , (4 , 2)]) ; let sccs : UsizeSccs = Sccs :: new (& graph) ; assert_eq ! (sccs . num_sccs () , 1) ; assert_eq ! (sccs . scc (0) , 0) ; assert_eq ! (sccs . scc (1) , 0) ; assert_eq ! (sccs . scc (2) , 0) ; assert_eq ! (sccs . scc (3) , 0) ; assert_eq ! (sccs . scc (4) , 0) ; assert_eq ! (sccs . successors (0) , & [] as & [usize]) ; }
};
}
