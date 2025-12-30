// Generated macro for find_source_directory (function)
macro_rules! Depcrate_persist_fsfind_source_directory {
() => {
// Module: crate::persist::fs
// Provides: {"find_source_directory"}
// Dependencies: {}
# [doc = " Finds the most recent published session directory that is not in the"] # [doc = " ignore-list."] fn find_source_directory (crate_dir : & Path , source_directories_already_tried : & FxHashSet < PathBuf > ,) -> Option < PathBuf > { let iter = crate_dir . read_dir () . unwrap () . filter_map (| e | e . ok () . map (| e | e . path ())) ; find_source_directory_in_iter (iter , source_directories_already_tried) }
};
}
