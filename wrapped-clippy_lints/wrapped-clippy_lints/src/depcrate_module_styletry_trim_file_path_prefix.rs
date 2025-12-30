// Generated macro for try_trim_file_path_prefix (function)
macro_rules! Depcrate_module_styletry_trim_file_path_prefix {
() => {
// Module: crate::module_style
// Provides: {"try_trim_file_path_prefix"}
// Dependencies: {}
fn try_trim_file_path_prefix < 'a > (file : & 'a SourceFile , prefix : & 'a Path) -> Option < & 'a Path > { if let FileName :: Real (name) = & file . name && let Some (mut path) = name . local_path () && file . cnum == LOCAL_CRATE { if ! path . is_relative () { path = path . strip_prefix (prefix) . ok () ? ; } Some (path) } else { None } }
};
}
