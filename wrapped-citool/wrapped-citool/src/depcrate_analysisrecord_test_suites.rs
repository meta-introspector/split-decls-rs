// Generated macro for record_test_suites (function)
macro_rules! Depcrate_analysisrecord_test_suites {
() => {
// Module: crate::analysis
// Provides: {"record_test_suites"}
// Dependencies: {}
fn record_test_suites (metrics : & JsonRoot) { let suites = metrics :: get_test_suites (& metrics) ; if ! suites . is_empty () { let aggregated = aggregate_test_suites (& suites) ; let table = render_table (aggregated) ; println ! ("\n# Test results\n") ; println ! ("{table}") ; } else { eprintln ! ("No test suites found in metrics") ; } }
};
}
