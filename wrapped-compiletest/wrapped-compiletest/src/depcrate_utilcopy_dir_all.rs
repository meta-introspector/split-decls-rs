// Generated macro for copy_dir_all (function)
macro_rules! Depcrate_utilcopy_dir_all {
() => {
// Module: crate::util
// Provides: {"copy_dir_all"}
// Dependencies: {}
pub fn copy_dir_all (src : & Utf8Path , dst : & Utf8Path) -> std :: io :: Result < () > { std :: fs :: create_dir_all (dst . as_std_path ()) ? ; for entry in std :: fs :: read_dir (src . as_std_path ()) ? { let entry = entry ? ; let path = Utf8PathBuf :: try_from (entry . path ()) . unwrap () ; let file_name = path . file_name () . unwrap () ; let ty = entry . file_type () ? ; if ty . is_dir () { copy_dir_all (& path , & dst . join (file_name)) ? ; } else { std :: fs :: copy (path . as_std_path () , dst . join (file_name) . as_std_path ()) ? ; } } Ok (()) }
};
}
