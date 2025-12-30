// Generated macro for upload_ci_metrics (function)
macro_rules! Depcrateupload_ci_metrics {
() => {
// Module: crate
// Provides: {"upload_ci_metrics"}
// Dependencies: {}
fn upload_ci_metrics (cpu_usage_csv : & Path) -> anyhow :: Result < () > { let usage = load_cpu_usage (cpu_usage_csv) . context ("Cannot load CPU usage from input CSV") ? ; eprintln ! ("CPU usage\n{usage:?}") ; let avg = if ! usage . is_empty () { usage . iter () . sum :: < f64 > () / usage . len () as f64 } else { 0.0 } ; eprintln ! ("CPU usage average: {avg}") ; upload_datadog_metric ("avg-cpu-usage" , avg) . context ("Cannot upload Datadog metric") ? ; Ok (()) }
};
}
