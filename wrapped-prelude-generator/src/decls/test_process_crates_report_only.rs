macro_rules! deps {
    () => {
        Args!();
        FileProcessingResult!();
        FileProcessingStatus!();
    };
}

macro_rules! test_process_crates_report_only {
    () => {
        deps!();
        # [test] fn test_process_crates_report_only () -> Result < () > { let temp_dir = tempdir () ? ; let project_root = temp_dir . path () . to_path_buf () ; let dummy_results = vec ! [FileProcessingResult { path : PathBuf :: from ("dummy/file.rs") , status : FileProcessingStatus :: Success , }] ; let results_json_path = project_root . join ("dummy_results.json") ; fs :: write (& results_json_path , serde_json :: to_string_pretty (& dummy_results) ? ,) ? ; let args = Args { dry_run : false , path : project_root . clone () , exclude_crates : vec ! [] , report : true , results_file : Some (results_json_path . clone ()) , cache_report : false , timeout : None , force : false , .. Default :: default () } ; process_crates (& args) ? ; let report_path = project_root . join ("prelude_generator_summary.md") ; assert ! (report_path . exists ()) ; let report_content = fs :: read_to_string (& report_path) ? ; assert ! (report_content . contains ("Prelude Generation Summary Report")) ; assert ! (report_content . contains ("dummy/file.rs")) ; Ok (()) }
    };
}

test_process_crates_report_only!();