// Generated macro for test_max_branch (function)
macro_rules! Depcrate_graph_scc_teststest_max_branch {
() => {
// Module: crate::graph::scc::tests
// Provides: {"test_max_branch"}
// Dependencies: {}
# [test] fn test_max_branch () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 4)]) ; let mut annotations = Maxes (IndexVec :: new () , | n | n) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . 0 [sccs . scc (0)] , 4) ; assert_eq ! (annotations . 0 [sccs . scc (1)] , 3) ; assert_eq ! (annotations . 0 [sccs . scc (2)] , 4) ; }
};
}
