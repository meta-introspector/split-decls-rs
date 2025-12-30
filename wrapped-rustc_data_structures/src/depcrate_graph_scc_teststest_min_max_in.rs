// Generated macro for test_min_max_in (function)
macro_rules! Depcrate_graph_scc_teststest_min_max_in {
() => {
// Module: crate::graph::scc::tests
// Provides: {"test_min_max_in"}
// Dependencies: {}
# [test] fn test_min_max_in () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (3 , 0) , (3 , 4) , (4 , 3) , (3 , 5)]) ; let mut annotations = MinMaxes (IndexVec :: new () , | w | MinMaxIn { min : w , max : w }) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . annotation (sccs . scc (2)) . min , 2) ; assert_eq ! (annotations . annotation (sccs . scc (2)) . max , 2) ; assert_eq ! (annotations . annotation (sccs . scc (0)) . min , 0) ; assert_eq ! (annotations . annotation (sccs . scc (0)) . max , 4) ; assert_eq ! (annotations . annotation (sccs . scc (3)) . min , 0) ; assert_eq ! (annotations . annotation (sccs . scc (3)) . max , 4) ; assert_eq ! (annotations . annotation (sccs . scc (5)) . min , 5) ; }
};
}
