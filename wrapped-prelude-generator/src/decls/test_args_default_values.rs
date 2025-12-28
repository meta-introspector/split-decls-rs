macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! test_args_default_values {
    () => {
        deps!();
        pub fn test_args_default_values () { let args = Args :: parse_from (& ["prelude-generator"]) ; assert ! (! args . dry_run) ; assert_eq ! (args . path , PathBuf :: from (".")) ; assert ! (args . exclude_crates . is_empty ()) ; assert ! (! args . report) ; assert_eq ! (args . results_file , Some (PathBuf :: from ("prelude_processing_results.json"))) ; assert ! (! args . cache_report) ; assert ! (args . timeout . is_none ()) ; assert ! (! args . force) ; }
    };
}

test_args_default_values!()