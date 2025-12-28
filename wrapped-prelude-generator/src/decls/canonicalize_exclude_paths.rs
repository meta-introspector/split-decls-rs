macro_rules! canonicalize_exclude_paths {
    () => {
        fn canonicalize_exclude_paths (exclude_paths : & [PathBuf] , project_root : & Path ,) -> anyhow :: Result < HashSet < PathBuf > > { let mut canonicalized_set = HashSet :: new () ; for path in exclude_paths { let absolute_path = if path . is_absolute () { path . clone () } else { project_root . join (path) } ; canonicalized_set . insert (canonicalize_path_buf (& absolute_path) ?) ; } Ok (canonicalized_set) }
    };
}

canonicalize_exclude_paths!();