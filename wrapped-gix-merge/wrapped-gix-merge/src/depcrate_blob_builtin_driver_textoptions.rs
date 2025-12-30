// Generated macro for Options (struct)
macro_rules! Depcrate_blob_builtin_driver_textOptions {
() => {
// Module: crate::blob::builtin_driver::text
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for the builtin [text driver](crate::blob::BuiltinDriver::Text)."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Options { # [doc = " Determine of the diff will be performed."] # [doc = " Defaults to [`imara_diff::Algorithm::Myers`]."] pub diff_algorithm : imara_diff :: Algorithm , # [doc = " Decide what to do to automatically resolve conflicts, or to keep them."] pub conflict : Conflict , }
};
}
