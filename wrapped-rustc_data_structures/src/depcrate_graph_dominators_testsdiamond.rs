// Generated macro for diamond (function)
macro_rules! Depcrate_graph_dominators_testsdiamond {
() => {
// Module: crate::graph::dominators::tests
// Provides: {"diamond"}
// Dependencies: {}
# [test] fn diamond () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3)]) ; let d = dominators (& graph) ; assert_eq ! (d . immediate_dominator (0) , None) ; assert_eq ! (d . immediate_dominator (1) , Some (0)) ; assert_eq ! (d . immediate_dominator (2) , Some (0)) ; assert_eq ! (d . immediate_dominator (3) , Some (0)) ; }
};
}
