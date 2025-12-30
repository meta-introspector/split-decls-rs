// Generated macro for try_unlink_path_recursively (function)
macro_rules! Depcrate_checkout_entrytry_unlink_path_recursively {
() => {
// Module: crate::checkout::entry
// Provides: {"try_unlink_path_recursively"}
// Dependencies: {}
fn try_unlink_path_recursively (path : & Path , path_meta : & std :: fs :: Metadata) -> std :: io :: Result < () > { if path_meta . is_dir () { std :: fs :: remove_dir_all (path) } else if path_meta . file_type () . is_symlink () { gix_fs :: symlink :: remove (path) } else { std :: fs :: remove_file (path) } }
};
}
