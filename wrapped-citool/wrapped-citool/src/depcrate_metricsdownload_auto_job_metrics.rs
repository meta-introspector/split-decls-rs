// Generated macro for download_auto_job_metrics (function)
macro_rules! Depcrate_metricsdownload_auto_job_metrics {
() => {
// Module: crate::metrics
// Provides: {"download_auto_job_metrics"}
// Dependencies: {}
# [doc = " Download before/after metrics for all auto jobs in the job database."] # [doc = " `parent` and `current` should be commit SHAs."] pub fn download_auto_job_metrics (job_db : & JobDatabase , parent : Option < & str > , current : & str ,) -> anyhow :: Result < HashMap < JobName , JobMetrics > > { let mut jobs = HashMap :: default () ; for job in & job_db . auto_jobs { eprintln ! ("Downloading metrics of job {}" , job . name) ; let metrics_parent = parent . and_then (| parent | match download_job_metrics (& job . name , parent) { Ok (metrics) => Some (metrics) , Err (error) => { eprintln ! (r#"Did not find metrics for job `{}` at `{parent}`: {error:?}.
Maybe it was newly added?"# , job . name) ; None } }) ; let metrics_current = download_job_metrics (& job . name , current) ? ; jobs . insert (job . name . clone () , JobMetrics { parent : metrics_parent , current : metrics_current } ,) ; } Ok (jobs) }
};
}
