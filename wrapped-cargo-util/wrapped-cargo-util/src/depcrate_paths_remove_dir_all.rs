// Generated macro for _remove_dir_all (function)
macro_rules! Depcrate_paths_remove_dir_all {
() => {
// Module: crate::paths
// Provides: {"_remove_dir_all"}
// Dependencies: {}
fn _remove_dir_all (p : & Path) -> Result < () > { if symlink_metadata (p) ? . is_symlink () { return remove_file (p) ; } let entries = p . read_dir () . with_context (| | format ! ("failed to read directory `{}`" , p . display ())) ? ; for entry in entries { let entry = entry ? ; let path = entry . path () ; if entry . file_type () ? . is_dir () { remove_dir_all (& path) ? ; } else { remove_file (& path) ? ; } } remove_dir (& p) }
};
}
