macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! read_config {
    () => {
        deps!();
        pub fn read_config (config_path : & Path , project_root : & Path) -> Result < Config > { let full_config_path = project_root . join (config_path) ; let config_content = std :: fs :: read_to_string (& full_config_path) . with_context (| | { format ! ("Failed to read config file from {}" , full_config_path . display ()) }) ? ; let config : Config = toml :: from_str (& config_content) . with_context (| | { format ! ("Failed to parse config file from {}" , full_config_path . display ()) }) ? ; Ok (config) }
    };
}

read_config!()