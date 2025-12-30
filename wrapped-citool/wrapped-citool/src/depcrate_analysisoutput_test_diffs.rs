// Generated macro for output_test_diffs (function)
macro_rules! Depcrate_analysisoutput_test_diffs {
() => {
// Module: crate::analysis
// Provides: {"output_test_diffs"}
// Dependencies: {}
# [doc = " Outputs a report of test differences between the `parent` and `current` commits."] pub fn output_test_diffs (job_metrics : & HashMap < JobName , JobMetrics > , job_info_resolver : & mut JobInfoResolver ,) { let aggregated_test_diffs = aggregate_test_diffs (& job_metrics) ; report_test_diffs (aggregated_test_diffs , job_metrics , job_info_resolver) ; }
};
}
