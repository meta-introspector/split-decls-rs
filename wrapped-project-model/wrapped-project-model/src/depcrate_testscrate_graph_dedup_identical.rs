// Generated macro for crate_graph_dedup_identical (function)
macro_rules! Depcrate_testscrate_graph_dedup_identical {
() => {
// Module: crate::tests
// Provides: {"crate_graph_dedup_identical"}
// Dependencies: {}
# [test] fn crate_graph_dedup_identical () { let (mut crate_graph , proc_macros) = load_cargo ("regex-metadata.json") ; let (d_crate_graph , mut d_proc_macros) = (crate_graph . clone () , proc_macros . clone ()) ; crate_graph . extend (d_crate_graph . clone () , & mut d_proc_macros) ; assert ! (crate_graph . iter () . eq (d_crate_graph . iter ())) ; assert_eq ! (proc_macros , d_proc_macros) ; }
};
}
