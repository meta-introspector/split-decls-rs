// Generated macro for each_node (function)
macro_rules! Depcrate_graph_linked_graph_testseach_node {
() => {
// Module: crate::graph::linked_graph::tests
// Provides: {"each_node"}
// Dependencies: {}
# [test] fn each_node () { let graph = create_graph () ; let expected = ["A" , "B" , "C" , "D" , "E" , "F"] ; graph . each_node (| idx , node | { assert_eq ! (& expected [idx . 0] , graph . node_data (idx)) ; assert_eq ! (expected [idx . 0] , node . data) ; true }) ; }
};
}
