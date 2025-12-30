// Generated macro for delete_directory (function)
macro_rules! Depcrate_utils_iodelete_directory {
() => {
// Module: crate::utils::io
// Provides: {"delete_directory"}
// Dependencies: {}
pub fn delete_directory (path : & Utf8Path) -> anyhow :: Result < () > { log :: info ! ("Deleting directory `{path}`") ; std :: fs :: remove_dir_all (path . as_std_path ()) . context (format ! ("Cannot remove directory {path}")) ? ; Ok (()) }
};
}
