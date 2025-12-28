macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! handle_extract_string_constants {
    () => {
        deps!();
        pub async fn handle_extract_string_constants (_project_root : & PathBuf , _args : & crate :: Args , all_string_constants : & Vec < syn :: ItemConst > ,) -> anyhow :: Result < () > { let string_output_dir = _args . generated_decls_output_dir . clone () . unwrap () . join ("string_constants") ; tokio :: fs :: create_dir_all (& string_output_dir) . await . context (format ! ("Failed to create output directory {:?}" , string_output_dir)) ? ; crate :: constant_storage :: string_constants :: write_string_constants_to_hierarchical_structure (& all_string_constants , & string_output_dir ,) . await ? ; println ! (r"  -> String constants will be written to: {:?}" , string_output_dir) ; println ! (r"  -> Total string constants extracted: {}" , all_string_constants . len ()) ; Ok (()) }
    };
}

handle_extract_string_constants!();