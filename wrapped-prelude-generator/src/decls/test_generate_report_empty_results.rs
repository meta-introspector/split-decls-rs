macro_rules! test_generate_report_empty_results {
    () => {
        pub fn test_generate_report_empty_results () -> Result < () > { let dir = tempdir () ? ; let original_dir = std :: env :: current_dir () ? ; std :: env :: set_current_dir (& dir) ? ; super :: report :: generate_report (& []) ? ; let report_path = dir . path () . join ("prelude_generator_summary.md") ; assert ! (report_path . exists ()) ; let content = fs :: read_to_string (& report_path) ? ; assert ! (content . contains ("# Prelude Generation Summary Report")) ; assert ! (content . contains ("- Total files processed: 0")) ; assert ! (! content . contains ("## Detailed Results")) ; std :: env :: set_current_dir (& original_dir) ? ; Ok (()) }
    };
}

test_generate_report_empty_results!()