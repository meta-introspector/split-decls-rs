// Generated macro for estimates (function)
macro_rules! Depcrate_analysis_compareestimates {
() => {
// Module: crate::analysis::compare
// Provides: {"estimates"}
// Dependencies: {}
fn estimates < M : Measurement > (id : & BenchmarkId , avg_times : & Sample < f64 > , base_avg_times : & Sample < f64 > , config : & BenchmarkConfig , criterion : & Criterion < M > ,) -> (ChangeEstimates , ChangeDistributions) { fn stats (a : & Sample < f64 > , b : & Sample < f64 >) -> (f64 , f64) { (a . mean () / b . mean () - 1. , a . percentiles () . median () / b . percentiles () . median () - 1. ,) } let cl = config . confidence_level ; let nresamples = config . nresamples ; let (dist_mean , dist_median) = elapsed ! ("Bootstrapping the relative statistics" , univariate :: bootstrap (avg_times , base_avg_times , nresamples , stats)) ; let distributions = ChangeDistributions { mean : dist_mean , median : dist_median , } ; let (mean , median) = stats (avg_times , base_avg_times) ; let points = ChangePointEstimates { mean , median } ; let estimates = build_change_estimates (& distributions , & points , cl) ; { log_if_err ! ({ let mut estimates_path = criterion . output_directory . clone () ; estimates_path . push (id . as_directory_name ()) ; estimates_path . push ("change") ; estimates_path . push ("estimates.json") ; fs :: save (& estimates , & estimates_path) }) ; } (estimates , distributions) }
};
}
