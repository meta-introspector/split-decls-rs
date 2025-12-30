// Generated macro for aggregate_test_suites (function)
macro_rules! Depcrate_analysisaggregate_test_suites {
() => {
// Module: crate::analysis
// Provides: {"aggregate_test_suites"}
// Dependencies: {}
fn aggregate_test_suites (suites : & [& TestSuite]) -> BTreeMap < String , TestSuiteRecord > { let mut records : BTreeMap < String , TestSuiteRecord > = BTreeMap :: new () ; for suite in suites { let name = test_metadata_name (& suite . metadata) ; let record = records . entry (name) . or_default () ; for test in & suite . tests { match test . outcome { TestOutcome :: Passed => { record . passed += 1 ; } TestOutcome :: Failed => { record . failed += 1 ; } TestOutcome :: Ignored { .. } => { record . ignored += 1 ; } } } } records }
};
}
