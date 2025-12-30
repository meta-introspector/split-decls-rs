// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> anyhow :: Result < () > { let args = Args :: parse () ; let default_jobs_file = Path :: new (JOBS_YML_PATH) ; let load_db = | jobs_path | { let db = utils :: read_to_string (jobs_path) ? ; Ok :: < _ , anyhow :: Error > (jobs :: load_job_db (& db) . context ("Cannot load jobs.yml") ?) } ; match args { Args :: CalculateJobMatrix { jobs_file } => { let jobs_path = jobs_file . as_deref () . unwrap_or (default_jobs_file) ; let gh_ctx = load_github_ctx () . context ("Cannot load environment variables from GitHub Actions") ? ; let channel = utils :: read_to_string (Path :: new (CI_DIRECTORY) . join ("channel")) . context ("Cannot read channel file") ? . trim () . to_string () ; jobs :: calculate_job_matrix (load_db (jobs_path) ? , gh_ctx , & channel) . context ("Failed to calculate job matrix") ? ; } Args :: RunJobLocally { job_type , name } => { run_workflow_locally (load_db (default_jobs_file) ? , job_type , name) ? ; } Args :: UploadBuildMetrics { cpu_usage_csv } => { upload_ci_metrics (& cpu_usage_csv) ? ; } Args :: PostprocessMetrics { metrics_path , parent , job_name } => { postprocess_metrics (metrics_path , parent , job_name) ? ; } Args :: PostMergeReport { current , parent } => { post_merge_report (load_db (& default_jobs_file) ? , current , parent) ? ; } Args :: TestDashboard { current , output_dir } => { let db = load_db (& default_jobs_file) ? ; generate_test_dashboard (db , & current , & output_dir) ? ; } } Ok (()) }
};
}
