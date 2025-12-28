macro_rules! find_source_directory {
    () => {
        # [doc = " Finds the most recent published session directory that is not in the"] # [doc = " ignore-list."] fn find_source_directory (crate_dir : & Path , source_directories_already_tried : & FxHashSet < PathBuf > ,) -> Option < PathBuf > { let iter = crate_dir . read_dir () . unwrap () . filter_map (| e | e . ok () . map (| e | e . path ())) ; find_source_directory_in_iter (iter , source_directories_already_tried) }
    };
}

find_source_directory!();