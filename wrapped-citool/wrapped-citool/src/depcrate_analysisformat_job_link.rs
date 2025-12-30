// Generated macro for format_job_link (function)
macro_rules! Depcrate_analysisformat_job_link {
() => {
// Module: crate::analysis
// Provides: {"format_job_link"}
// Dependencies: {}
# [doc = " Tries to get a GitHub Actions job summary URL from the resolver."] # [doc = " If it is not available, just wraps the job name in backticks."] fn format_job_link (job_info_resolver : & mut JobInfoResolver , job_metrics : & HashMap < JobName , JobMetrics > , job_name : & str ,) -> String { job_metrics . get (job_name) . and_then (| metrics | job_info_resolver . get_job_summary_link (job_name , & metrics . current)) . map (| summary_url | format ! ("[{job_name}]({summary_url})")) . unwrap_or_else (| | format ! ("`{job_name}`")) }
};
}
