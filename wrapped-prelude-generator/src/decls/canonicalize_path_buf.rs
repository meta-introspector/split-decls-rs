macro_rules! canonicalize_path_buf {
    () => {
        fn canonicalize_path_buf (path : & Path) -> anyhow :: Result < PathBuf > { path . canonicalize () . with_context (| | format ! ("Failed to canonicalize path: {}" , path . display ())) }
    };
}

canonicalize_path_buf!()