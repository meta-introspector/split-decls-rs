// Generated macro for postprocess_metrics (function)
macro_rules! Depcratepostprocess_metrics {
() => {
// Module: crate
// Provides: {"postprocess_metrics"}
// Dependencies: {}
fn postprocess_metrics (metrics_path : PathBuf , parent : Option < String > , job_name : Option < String > ,) -> anyhow :: Result < () > { let metrics = load_metrics (& metrics_path) ? ; let mut job_info_resolver = JobInfoResolver :: new () ; if let (Some (parent) , Some (job_name)) = (parent , job_name) { match download_job_metrics (& job_name , & parent) . context ("cannot download parent metrics") { Ok (parent_metrics) => { output_bootstrap_stats (& metrics , Some (& parent_metrics)) ; let job_metrics = HashMap :: from ([(job_name , JobMetrics { parent : Some (parent_metrics) , current : metrics } ,)]) ; output_test_diffs (& job_metrics , & mut job_info_resolver) ; return Ok (()) ; } Err (error) => { eprintln ! ("Metrics for job `{job_name}` and commit `{parent}` not found: {error:?}") ; } } } output_bootstrap_stats (& metrics , None) ; Ok (()) }
};
}
