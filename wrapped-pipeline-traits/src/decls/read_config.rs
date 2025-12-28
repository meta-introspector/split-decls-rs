macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! read_config {
    () => {
        deps!();
        pub fn read_config (config_path : & PathBuf , project_root : & PathBuf) -> Result < Config > { let config_content = std :: fs :: read_to_string (config_path) . with_context (| | format ! ("Failed to read config file: {}" , config_path . display ())) ? ; let mut config : Config = toml :: from_str (& config_content) . with_context (| | format ! ("Failed to parse config file: {}" , config_path . display ())) ? ; if let Some (bins_config) = & mut config . bins { for (_ , path) in bins_config . paths . iter_mut () { if ! path . is_absolute () { * path = project_root . join (& path) ; } } } if let Some (generated_output_dir) = & mut config . generated_output_dir { if ! generated_output_dir . is_absolute () { * generated_output_dir = project_root . join (& * generated_output_dir) ; } } Ok (config) }
    };
}

read_config!()