macro_rules! system_prefix_from_core_dir {
    () => {
        fn system_prefix_from_core_dir < F > (core_dir_func : F) -> Option < PathBuf > where F : Fn () -> Option < & 'static Path > , { let path = core_dir_func () ? ; let one_past_prefix = path . components () . enumerate () . find_map (| (idx , c) | { matches ! (c , std :: path :: Component :: Normal (name) if name . to_str () == Some ("libexec")) . then_some (idx) }) ? ; Some (path . components () . take (one_past_prefix . checked_sub (1) ?) . collect ()) }
    };
}

system_prefix_from_core_dir!();