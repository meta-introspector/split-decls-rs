// Generated macro for impl_204 (impl)
macro_rules! Depcrate_info_filetypeimpl_204 {
() => {
// Module: crate::info::filetype
// Provides: {"impl_204"}
// Dependencies: {}
impl FileType { # [doc = " Lookup the file type based on the file's name, by the file name"] # [doc = " lowercase extension, or if the file could be compiled from related"] # [doc = " source code."] pub (crate) fn get_file_type (file : & File < '_ >) -> Option < FileType > { if file . name . to_lowercase () . starts_with ("readme") { return Some (Self :: Build) ; } if let Some (file_type) = FILENAME_TYPES . get (& file . name) { return Some (file_type . clone ()) ; } if let Some (file_type) = file . ext . as_ref () . and_then (| ext | EXTENSION_TYPES . get (ext)) { return Some (file_type . clone ()) ; } if file . name . ends_with ('~') || (file . name . starts_with ('#') && file . name . ends_with ('#')) { return Some (Self :: Temp) ; } if let Some (dir) = file . parent_dir { if file . get_source_files () . iter () . any (| path | dir . contains (path)) { return Some (Self :: Compiled) ; } } None } }
};
}
