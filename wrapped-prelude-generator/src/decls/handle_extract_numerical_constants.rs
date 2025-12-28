macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! handle_extract_numerical_constants {
    () => {
        deps!();
        pub async fn handle_extract_numerical_constants (_project_root : & PathBuf , _args : & crate :: Args , all_numerical_constants : & Vec < syn :: ItemConst > ,) -> anyhow :: Result < () > { let numerical_output_dir = _args . generated_decls_output_dir . clone () . unwrap () . join ("numerical_constants") ; tokio :: fs :: create_dir_all (& numerical_output_dir) . await . context (format ! ("Failed to create output directory {:?}" , numerical_output_dir)) ? ; write_numerical_constants_to_hierarchical_structure (& all_numerical_constants , & numerical_output_dir ,) . await ? ; println ! (r"  -> Numerical constants will be written to: {:?}" , numerical_output_dir) ; println ! (r"  -> Total numerical constants extracted: {}" , all_numerical_constants . len ()) ; Ok (()) }
    };
}

handle_extract_numerical_constants!();