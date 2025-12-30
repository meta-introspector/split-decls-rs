// Generated macro for base_dir_exists (function)
macro_rules! Depcrate_analysisbase_dir_exists {
() => {
// Module: crate::analysis
// Provides: {"base_dir_exists"}
// Dependencies: {}
fn base_dir_exists (id : & BenchmarkId , baseline : & str , output_directory : & Path) -> bool { let mut base_dir = output_directory . to_owned () ; base_dir . push (id . as_directory_name ()) ; base_dir . push (baseline) ; base_dir . exists () }
};
}
