macro_rules! deps {
    () => {
        FileProcessingResult!();
        FileProcessingStatus!();
    };
}

macro_rules! test_generate_report_with_results {
    () => {
        deps!();
        pub fn test_generate_report_with_results () -> Result < () > { let dir = tempdir () ? ; let original_dir = std :: env :: current_dir () ? ; std :: env :: set_current_dir (& dir) ? ; let results = vec ! [FileProcessingResult { path : PathBuf :: from ("src/file1.rs") , status : FileProcessingStatus :: Success , } , FileProcessingResult { path : PathBuf :: from ("src/file2.rs") , status : FileProcessingStatus :: Skipped { reason : "already processed" . to_string () , } , } , FileProcessingResult { path : PathBuf :: from ("src/file3.rs") , status : FileProcessingStatus :: Failed { error : "syntax error" . to_string () , } , } ,] ; super :: report :: generate_report (& results) ? ; let report_path = dir . path () . join ("prelude_generator_summary.md") ; assert ! (report_path . exists ()) ; let content = fs :: read_to_string (& report_path) ? ; assert ! (content . contains ("# Prelude Generation Summary Report")) ; assert ! (content . contains ("- Total files processed: 3")) ; assert ! (content . contains ("- Successfully processed: 1")) ; assert ! (content . contains ("- Skipped: 1")) ; assert ! (content . contains ("- Failed: 1")) ; assert ! (content . contains ("### src/file1.rs\n- Status: ✅ Successfully Processed")) ; assert ! (content . contains ("### src/file2.rs\n- Status: ⏭️ Skipped (Reason: already processed")) ; assert ! (content . contains ("### src/file3.rs\n- Status: ❌ Failed (Error: syntax error")) ; std :: env :: set_current_dir (& original_dir) ? ; Ok (()) }
    };
}

test_generate_report_with_results!()