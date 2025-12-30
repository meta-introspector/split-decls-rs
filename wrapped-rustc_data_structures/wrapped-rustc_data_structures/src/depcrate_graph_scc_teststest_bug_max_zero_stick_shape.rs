// Generated macro for test_bug_max_zero_stick_shape (function)
macro_rules! Depcrate_graph_scc_teststest_bug_max_zero_stick_shape {
() => {
// Module: crate::graph::scc::tests
// Provides: {"test_bug_max_zero_stick_shape"}
// Dependencies: {}
# [test] fn test_bug_max_zero_stick_shape () { let graph = TestGraph :: new (0 , & [(0 , 1) , (1 , 2) , (2 , 3) , (3 , 2) , (3 , 4)]) ; let mut annotations = Maxes :: new (| w | match w { 4 => 1 , _ => 0 , }) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . annotation (sccs . scc (0)) , 1) ; assert_eq ! (annotations . annotation (sccs . scc (1)) , 1) ; assert_eq ! (annotations . annotation (sccs . scc (2)) , 1) ; assert_eq ! (annotations . annotation (sccs . scc (3)) , 1) ; assert_eq ! (annotations . annotation (sccs . scc (4)) , 1) ; }
};
}
