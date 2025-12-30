// Generated macro for read_dir_entries_recursive (function)
macro_rules! Depcrate_path_helpersread_dir_entries_recursive {
() => {
// Module: crate::path_helpers
// Provides: {"read_dir_entries_recursive"}
// Dependencies: {}
# [doc = " Helper for reading entries in a given directory and its children."] pub fn read_dir_entries_recursive < P : AsRef < Path > , F : FnMut (& Path) > (dir : P , mut callback : F) { fn read_dir_entries_recursive_inner < P : AsRef < Path > , F : FnMut (& Path) > (dir : P , callback : & mut F) { for entry in rfs :: read_dir (dir) { let path = entry . unwrap () . path () ; callback (& path) ; if path . is_dir () { read_dir_entries_recursive_inner (path , callback) ; } } } read_dir_entries_recursive_inner (dir , & mut callback) ; }
};
}
