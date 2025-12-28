macro_rules! deps {
    () => {
        Config!();
        Args!();
    };
}

macro_rules! parse_arguments_and_config {
    () => {
        deps!();
        pub fn parse_arguments_and_config () -> anyhow :: Result < (Args , Option < Config >) > { let args = Args :: parse () ; let project_root = if args . path == PathBuf :: from (".") { std :: env :: current_dir () ? . parent () . unwrap () . to_path_buf () } else { PathBuf :: from (& args . path) } ; let config = if let Some (config_file_path) = & args . config_file_path { Some (read_config (config_file_path , & project_root) ?) } else { let default_config_path = project_root . join ("config.toml") ; if default_config_path . exists () { Some (read_config (& default_config_path , & project_root) ?) } else { None } } ; Ok ((args , config)) }
    };
}

parse_arguments_and_config!()