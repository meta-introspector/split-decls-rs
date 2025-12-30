// Generated macro for test_deep_linear (function)
macro_rules! Depcrate_graph_scc_teststest_deep_linear {
() => {
// Module: crate::graph::scc::tests
// Provides: {"test_deep_linear"}
// Dependencies: {}
# [test] fn test_deep_linear () { # [cfg (not (miri))] const NR_NODES : usize = 1 << 14 ; # [cfg (miri)] const NR_NODES : usize = 1 << 3 ; let mut nodes = vec ! [] ; for i in 1 .. NR_NODES { nodes . push ((i - 1 , i)) ; } let graph = TestGraph :: new (0 , nodes . as_slice ()) ; let sccs : UsizeSccs = Sccs :: new (& graph) ; assert_eq ! (sccs . num_sccs () , NR_NODES) ; assert_eq ! (sccs . scc (0) , NR_NODES - 1) ; assert_eq ! (sccs . scc (NR_NODES - 1) , 0) ; }
};
}
