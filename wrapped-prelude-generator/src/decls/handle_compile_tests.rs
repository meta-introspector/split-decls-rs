macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! handle_compile_tests {
    () => {
        deps!();
        pub fn handle_compile_tests (_args : & crate :: Args) -> anyhow :: Result < () > { let _input_file = _args . test_report_input_file . clone () . ok_or_else (| | { anyhow :: anyhow ! ("test_report_input_file is required when compile_tests is true") }) ? ; let _output_dir = _args . test_verification_output_dir . clone () . ok_or_else (| | { anyhow :: anyhow ! ("test_verification_output_dir is required when compile_tests is true") }) ? ; Ok (()) }
    };
}

handle_compile_tests!()