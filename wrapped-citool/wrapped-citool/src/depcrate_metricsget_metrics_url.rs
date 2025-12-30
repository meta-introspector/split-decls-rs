// Generated macro for get_metrics_url (function)
macro_rules! Depcrate_metricsget_metrics_url {
() => {
// Module: crate::metrics
// Provides: {"get_metrics_url"}
// Dependencies: {}
fn get_metrics_url (job_name : & str , sha : & str) -> String { let suffix = if job_name . ends_with ("-alt") { "-alt" } else { "" } ; format ! ("https://ci-artifacts.rust-lang.org/rustc-builds{suffix}/{sha}/metrics-{job_name}.json") }
};
}
