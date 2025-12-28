macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! handle_extract_use_statements {
    () => {
        deps!();
        pub fn handle_extract_use_statements (_args : & crate :: Args) -> anyhow :: Result < () > { let output_dir = _args . use_statements_output_dir . clone () . ok_or_else (| | { anyhow :: anyhow ! ("use_statements_output_dir is required when extract_use_statements is true") }) ? ; println ! ("Extracting use statements to: {}" , output_dir . display ()) ; Ok (()) }
    };
}

handle_extract_use_statements!();