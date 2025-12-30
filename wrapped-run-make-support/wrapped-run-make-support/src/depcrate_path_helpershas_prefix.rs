// Generated macro for has_prefix (function)
macro_rules! Depcrate_path_helpershas_prefix {
() => {
// Module: crate::path_helpers
// Provides: {"has_prefix"}
// Dependencies: {}
# [doc = " Returns true if the filename at `path` starts with `prefix`."] pub fn has_prefix < P : AsRef < Path > > (path : P , prefix : & str) -> bool { path . as_ref () . file_name () . is_some_and (| name | name . to_str () . unwrap () . starts_with (prefix)) }
};
}
