macro_rules! add_lock_suffix {
    () => {
        fn add_lock_suffix (resource_path : & Path) -> PathBuf { resource_path . with_extension (resource_path . extension () . map_or_else (| | DOT_LOCK_SUFFIX . chars () . skip (1) . collect () , | ext | format ! ("{}{}" , ext . to_string_lossy () , DOT_LOCK_SUFFIX) ,)) }
    };
}

add_lock_suffix!();