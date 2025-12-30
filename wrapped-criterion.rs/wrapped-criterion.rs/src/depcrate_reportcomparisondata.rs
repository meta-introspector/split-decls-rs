// Generated macro for ComparisonData (struct)
macro_rules! Depcrate_reportComparisonData {
() => {
// Module: crate::report
// Provides: {"ComparisonData"}
// Dependencies: {}
pub (crate) struct ComparisonData { pub p_value : f64 , pub t_distribution : Distribution < f64 > , pub t_value : f64 , pub relative_estimates : ChangeEstimates , pub relative_distributions : ChangeDistributions , pub significance_threshold : f64 , pub noise_threshold : f64 , pub base_iter_counts : Vec < f64 > , pub base_sample_times : Vec < f64 > , pub base_avg_times : Vec < f64 > , pub base_estimates : Estimates , }
};
}
