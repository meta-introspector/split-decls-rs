// Generated macro for generate_test_dashboard (function)
macro_rules! Depcrate_test_dashboardgenerate_test_dashboard {
() => {
// Module: crate::test_dashboard
// Provides: {"generate_test_dashboard"}
// Dependencies: {}
# [doc = " Generate a set of HTML files into a directory that contain a dashboard of test results."] pub fn generate_test_dashboard (db : JobDatabase , current : & str , output_dir : & Path ,) -> anyhow :: Result < () > { let metrics = download_auto_job_metrics (& db , None , current) ? ; let suites = gather_test_suites (& metrics) ; std :: fs :: create_dir_all (output_dir) ? ; let test_count = suites . test_count () ; write_page (output_dir , "index.html" , & TestSuitesPage { suites , test_count }) ? ; Ok (()) }
};
}
