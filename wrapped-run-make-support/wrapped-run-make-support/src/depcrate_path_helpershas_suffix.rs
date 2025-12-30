// Generated macro for has_suffix (function)
macro_rules! Depcrate_path_helpershas_suffix {
() => {
// Module: crate::path_helpers
// Provides: {"has_suffix"}
// Dependencies: {}
# [doc = " Returns true if the filename at `path` ends with `suffix`."] pub fn has_suffix < P : AsRef < Path > > (path : P , suffix : & str) -> bool { path . as_ref () . file_name () . is_some_and (| name | name . to_str () . unwrap () . ends_with (suffix)) }
};
}
