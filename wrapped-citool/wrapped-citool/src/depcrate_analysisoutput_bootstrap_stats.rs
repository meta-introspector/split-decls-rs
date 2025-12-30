// Generated macro for output_bootstrap_stats (function)
macro_rules! Depcrate_analysisoutput_bootstrap_stats {
() => {
// Module: crate::analysis
// Provides: {"output_bootstrap_stats"}
// Dependencies: {}
# [doc = " Outputs durations of individual bootstrap steps from the gathered bootstrap invocations,"] # [doc = " and also a table with summarized information about executed tests."] pub fn output_bootstrap_stats (metrics : & JsonRoot , parent_metrics : Option < & JsonRoot >) { if ! metrics . invocations . is_empty () { println ! ("# Bootstrap steps") ; record_bootstrap_step_durations (& metrics , parent_metrics) ; record_test_suites (& metrics) ; } }
};
}
