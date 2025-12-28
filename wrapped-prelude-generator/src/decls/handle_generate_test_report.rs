macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! handle_generate_test_report {
    () => {
        deps!();
        pub fn handle_generate_test_report (_args : & crate :: Args) -> anyhow :: Result < () > { let _output_file = _args . test_report_output_file . clone () . unwrap_or_else (| | PathBuf :: from ("test_report.json")) ; Ok (()) }
    };
}

handle_generate_test_report!();