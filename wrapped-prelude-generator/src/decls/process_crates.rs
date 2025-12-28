macro_rules! deps {
    () => {
        FileProcessingResult!();
        Args!();
    };
}

macro_rules! process_crates {
    () => {
        deps!();
        pub fn process_crates (args : & Args) -> Result < () > { println ! ("--- Prelude Generator Started ---") ; println ! ("Parsed arguments: {:?}" , args) ; let _start_time = Instant :: now () ; let _timeout_duration = args . timeout . map (Duration :: from_secs) ; if args . cache_report { let cache_dir = PathBuf :: from (& args . path) . join (".prelude_cache") ; if cache_dir . exists () { let count = fs :: read_dir (& cache_dir) ? . count () ; println ! ("Prelude cache at {} contains {} items." , cache_dir . display () , count) ; } else { println ! ("Prelude cache directory not found at {}." , cache_dir . display ()) ; } return Ok (()) ; } let all_file_processing_results : Vec < FileProcessingResult > = Vec :: new () ; if args . report { if let Some (results_file_path) = & args . results_file { if results_file_path . exists () { let json_content = fs :: read_to_string (results_file_path) . context ("Failed to read results file") ? ; let results : Vec < FileProcessingResult > = serde_json :: from_str (& json_content) . context ("Failed to deserialize results from JSON") ? ; generate_report (& results) ? ; } else { eprintln ! ("Error: Results file not found at {}. Cannot generate report." , results_file_path . display ()) ; } } } else { let mut excluded_crates : HashSet < String > = args . exclude_crates . clone () . into_iter () . collect () ; excluded_crates . insert ("prelude-generator" . to_string ()) ; excluded_crates . insert ("rust-decl-splitter" . to_string ()) ; excluded_crates . insert ("dependency-analyzer" . to_string ()) ; excluded_crates . insert ("prelude-collector" . to_string ()) ; println ! ("Excluded crates: {:?}" , excluded_crates) ; println ! ("\nPrelude generation complete.") ; println ! ("  -> Contents of all_file_processing_results: {:?}" , all_file_processing_results) ; if let Some (results_file_path) = & args . results_file { let json_content = serde_json :: to_string_pretty (& all_file_processing_results) . context ("Failed to serialize results to JSON") ? ; println ! ("  -> Attempting to save processing results to: {}" , results_file_path . display ()) ; fs :: write (results_file_path , json_content) . context ("Failed to write results to file") ? ; println ! ("Processing results saved to {}." , results_file_path . display ()) ; } else { println ! ("No results file specified. Skipping saving processing results.") ; } } Ok (()) }
    };
}

process_crates!();