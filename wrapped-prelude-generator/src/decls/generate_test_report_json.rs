macro_rules! deps {
    () => {
        TestInfo!();
    };
}

macro_rules! generate_test_report_json {
    () => {
        deps!();
        # [doc = " Generates a JSON report of all collected test cases."] pub fn generate_test_report_json (output_path : & Path , test_functions : Vec < TestInfo >) -> Result < () > { let json_content = serde_json :: to_string_pretty (& test_functions) . context ("Failed to serialize test info to JSON") ? ; fs :: write (output_path , json_content) . with_context (| | { format ! ("Failed to write aggregated test report to {}" , output_path . display ()) }) ? ; println ! ("Aggregated test report generated at: {}" , output_path . display ()) ; Ok (()) }
    };
}

generate_test_report_json!();