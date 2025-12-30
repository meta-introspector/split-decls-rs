// Generated macro for deduplicate_paths (function)
macro_rules! Depcratededuplicate_paths {
() => {
// Module: crate
// Provides: {"deduplicate_paths"}
// Dependencies: {}
fn deduplicate_paths (copy : & mut PathBuf , original : & PathBuf) { if path_to_string (& * * copy) == path_to_string (original) { let new_path = match original . file_name () { Some (name) => { let mut new_name = name . to_owned () ; new_name . push ("_copy") ; copy . with_file_name (new_name) } None => copy . with_file_name ("copy") , } ; * copy = new_path ; } }
};
}
