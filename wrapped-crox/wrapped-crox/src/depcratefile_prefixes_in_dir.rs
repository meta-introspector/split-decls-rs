// Generated macro for file_prefixes_in_dir (function)
macro_rules! Depcratefile_prefixes_in_dir {
() => {
// Module: crate
// Provides: {"file_prefixes_in_dir"}
// Dependencies: {}
fn file_prefixes_in_dir (opt : & Opt) -> Result < Vec < PathBuf > , std :: io :: Error > { let mut result = Vec :: new () ; if let Some (dir_path) = & opt . dir { for entry in fs :: read_dir (dir_path) ? { let entry = entry ? ; let path = entry . path () ; if path . extension () . filter (| e | * e == FILE_EXTENSION) . is_some () { result . push (path) } } } Ok (result) }
};
}
