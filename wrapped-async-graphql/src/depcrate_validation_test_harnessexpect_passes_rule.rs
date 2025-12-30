// Generated macro for expect_passes_rule (macro)
macro_rules! Depcrate_validation_test_harnessexpect_passes_rule {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_passes_rule"}
// Dependencies: {}
macro_rules ! expect_passes_rule { ($ factory : expr , $ query_source : literal $ (,) ?) => { let doc = crate :: parser :: parse_query ($ query_source) . expect ("Parse error") ; crate :: validation :: test_harness :: expect_passes_rule_ (& doc , $ factory) ; } ; }
};
}
