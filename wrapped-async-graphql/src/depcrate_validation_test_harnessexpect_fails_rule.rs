// Generated macro for expect_fails_rule (macro)
macro_rules! Depcrate_validation_test_harnessexpect_fails_rule {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_fails_rule"}
// Dependencies: {}
macro_rules ! expect_fails_rule { ($ factory : expr , $ query_source : literal $ (,) ?) => { let doc = crate :: parser :: parse_query ($ query_source) . expect ("Parse error") ; crate :: validation :: test_harness :: expect_fails_rule_ (& doc , $ factory) ; } ; }
};
}
