macro_rules! deps {
    () => {
        TestInfo!();
    };
}

macro_rules! extract_test_cases_from_file {
    () => {
        deps!();
        # [doc = " Extracts test functions from a single Rust file."] fn extract_test_cases_from_file (file_path : & Path) -> Result < Vec < TestInfo > > { let content = fs :: read_to_string (file_path) . with_context (| | format ! ("Failed to read file: {}" , file_path . display ())) ? ; let ast = syn :: parse_file (& content) . with_context (| | format ! ("Failed to parse Rust file: {}" , file_path . display ())) ? ; Ok (extract_test_functions_from_items (ast . items , file_path)) }
    };
}

extract_test_cases_from_file!()