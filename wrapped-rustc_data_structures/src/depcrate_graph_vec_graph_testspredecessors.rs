// Generated macro for predecessors (function)
macro_rules! Depcrate_graph_vec_graph_testspredecessors {
() => {
// Module: crate::graph::vec_graph::tests
// Provides: {"predecessors"}
// Dependencies: {}
# [test] fn predecessors () { let graph = create_graph_with_back_refs () ; assert_eq ! (graph . predecessors (0) , & []) ; assert_eq ! (graph . predecessors (1) , & [0 , 5]) ; assert_eq ! (graph . predecessors (2) , & [1]) ; assert_eq ! (graph . predecessors (3) , & [1]) ; assert_eq ! (graph . predecessors (4) , & [3]) ; assert_eq ! (graph . predecessors (5) , & []) ; assert_eq ! (graph . predecessors (6) , & []) ; }
};
}
