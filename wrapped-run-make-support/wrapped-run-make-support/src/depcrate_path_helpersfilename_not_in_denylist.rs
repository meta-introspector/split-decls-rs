// Generated macro for filename_not_in_denylist (function)
macro_rules! Depcrate_path_helpersfilename_not_in_denylist {
() => {
// Module: crate::path_helpers
// Provides: {"filename_not_in_denylist"}
// Dependencies: {}
# [doc = " Returns true if the filename at `path` is not in `expected`."] pub fn filename_not_in_denylist < P : AsRef < Path > , V : AsRef < [String] > > (path : P , expected : V) -> bool { let expected = expected . as_ref () ; path . as_ref () . file_name () . is_some_and (| name | ! expected . contains (& name . to_str () . unwrap () . to_owned ())) }
};
}
