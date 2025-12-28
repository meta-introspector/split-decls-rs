macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! test_args_custom_values {
    () => {
        deps!();
        pub fn test_args_custom_values () { let args = Args :: parse_from (& ["prelude-generator" , "--dry-run" , "--path" , "/tmp/my_project" , "--exclude-crates" , "crate1,crate2" , "--report" , "--results-file" , "custom_results.json" , "--cache-report" , "--timeout" , "60" , "--force" ,]) ; assert ! (args . dry_run) ; assert_eq ! (args . path , PathBuf :: from ("/tmp/my_project")) ; assert_eq ! (args . exclude_crates , vec ! ["crate1" . to_string () , "crate2" . to_string ()]) ; assert ! (args . report) ; assert_eq ! (args . results_file , Some (PathBuf :: from ("custom_results.json"))) ; assert ! (args . cache_report) ; assert_eq ! (args . timeout , Some (60)) ; assert ! (args . force) ; }
    };
}

test_args_custom_values!()