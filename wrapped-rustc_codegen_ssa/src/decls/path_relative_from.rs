macro_rules! path_relative_from {
    () => {
        fn path_relative_from (path : & Path , base : & Path) -> Option < PathBuf > { diff_paths (path , base) }
    };
}

path_relative_from!();