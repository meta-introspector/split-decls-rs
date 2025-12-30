// Generated macro for aggregate_test_diffs (function)
macro_rules! Depcrate_analysisaggregate_test_diffs {
() => {
// Module: crate::analysis
// Provides: {"aggregate_test_diffs"}
// Dependencies: {}
fn aggregate_test_diffs (jobs : & HashMap < JobName , JobMetrics >) -> AggregatedTestDiffs { let mut diffs : HashMap < TestDiff , Vec < JobName > > = HashMap :: new () ; for (name , metrics) in jobs { if let Some (parent) = & metrics . parent { let tests_parent = aggregate_tests (parent) ; let tests_current = aggregate_tests (& metrics . current) ; for diff in calculate_test_diffs (tests_parent , tests_current) { diffs . entry (diff) . or_default () . push (name . to_string ()) ; } } } AggregatedTestDiffs { diffs } }
};
}
