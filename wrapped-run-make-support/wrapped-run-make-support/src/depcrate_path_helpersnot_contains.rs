// Generated macro for not_contains (function)
macro_rules! Depcrate_path_helpersnot_contains {
() => {
// Module: crate::path_helpers
// Provides: {"not_contains"}
// Dependencies: {}
# [doc = " Returns true if the filename at `path` does not contain `expected`."] pub fn not_contains < P : AsRef < Path > > (path : P , expected : & str) -> bool { ! path . as_ref () . file_name () . is_some_and (| name | name . to_str () . unwrap () . contains (expected)) }
};
}
