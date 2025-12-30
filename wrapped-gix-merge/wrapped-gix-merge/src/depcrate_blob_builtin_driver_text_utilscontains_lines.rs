// Generated macro for contains_lines (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilscontains_lines {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"contains_lines"}
// Dependencies: {}
pub fn contains_lines (hunks : & [Hunk]) -> bool { hunks . iter () . any (| h | ! h . after . is_empty ()) }
};
}
