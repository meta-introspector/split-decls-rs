// Generated macro for transitive_dominator (function)
macro_rules! Depcrate_graph_dominators_teststransitive_dominator {
() => {
// Module: crate::graph::dominators::tests
// Provides: {"transitive_dominator"}
// Dependencies: {}
# [test] fn transitive_dominator () { let graph = TestGraph :: new (0 , & [(0 , 1) , (1 , 2) , (2 , 3) , (3 , 4) , (1 , 5) , (5 , 6) , (0 , 7) , (7 , 2) , (5 , 3) ,] ,) ; let d = dominators (& graph) ; assert_eq ! (d . immediate_dominator (2) , Some (0)) ; assert_eq ! (d . immediate_dominator (3) , Some (0)) ; }
};
}
