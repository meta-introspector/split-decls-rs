// Generated macro for check_crate_graph (function)
macro_rules! Depcrate_testscheck_crate_graph {
() => {
// Module: crate::tests
// Provides: {"check_crate_graph"}
// Dependencies: {}
fn check_crate_graph (crate_graph : CrateGraphBuilder , expect : ExpectFile) { let mut crate_graph = format ! ("{crate_graph:#?}") ; replace_root (& mut crate_graph , false) ; replace_cargo (& mut crate_graph) ; expect . assert_eq (& crate_graph) ; }
};
}
