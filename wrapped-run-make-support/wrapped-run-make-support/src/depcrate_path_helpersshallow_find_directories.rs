// Generated macro for shallow_find_directories (function)
macro_rules! Depcrate_path_helpersshallow_find_directories {
() => {
// Module: crate::path_helpers
// Provides: {"shallow_find_directories"}
// Dependencies: {}
# [doc = " Browse the directory `path` non-recursively and return all directories which respect the"] # [doc = " parameters outlined by `closure`."] # [track_caller] pub fn shallow_find_directories < P : AsRef < Path > , F : Fn (& PathBuf) -> bool > (path : P , filter : F ,) -> Vec < PathBuf > { let mut matching_files = Vec :: new () ; for entry in rfs :: read_dir (path) { let entry = entry . expect ("failed to read directory entry.") ; let path = entry . path () ; if path . is_dir () && filter (& path) { matching_files . push (path) ; } } matching_files }
};
}
