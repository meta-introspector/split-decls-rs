macro_rules! dir_contains_target {
    () => {
        fn dir_contains_target (target : & Option < OsString > , dir : & Path , cargo_target_dir : Option < OsString > ,) -> bool { target . as_ref () . and_then (| target | { dir . to_str () . and_then (| dir | { let mut cargo_target_dir = cargo_target_dir . map (PathBuf :: from) . unwrap_or_else (| | PathBuf :: from ("target")) ; cargo_target_dir . push (target) ; cargo_target_dir . to_str () . map (| cargo_target_dir | dir . contains (cargo_target_dir)) }) }) . unwrap_or (false) }
    };
}

dir_contains_target!()