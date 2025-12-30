// Generated macro for empty_graph (function)
macro_rules! Depcrate_testsempty_graph {
() => {
// Module: crate::tests
// Provides: {"empty_graph"}
// Dependencies: {}
# [test] fn empty_graph () { let labels : Trivial = UnlabelledNodes (0) ; let r = test_input (LabelledGraph :: new ("empty_graph" , labels , vec ! [] , None)) ; assert_eq ! (r . unwrap () , r#"digraph empty_graph {
}
"#) ; }
};
}
