// Generated macro for filename_contains (function)
macro_rules! Depcrate_path_helpersfilename_contains {
() => {
// Module: crate::path_helpers
// Provides: {"filename_contains"}
// Dependencies: {}
# [doc = " Returns true if the filename at `path` contains `needle`."] pub fn filename_contains < P : AsRef < Path > > (path : P , needle : & str) -> bool { path . as_ref () . file_name () . is_some_and (| name | name . to_str () . unwrap () . contains (needle)) }
};
}
