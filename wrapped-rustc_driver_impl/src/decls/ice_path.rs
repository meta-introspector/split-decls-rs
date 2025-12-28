macro_rules! ice_path {
    () => {
        fn ice_path () -> & 'static Option < PathBuf > { ice_path_with_config (None) }
    };
}

ice_path!();