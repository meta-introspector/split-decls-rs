// Generated macro for out_dir (function)
macro_rules! Depcrate_inputout_dir {
() => {
// Module: crate::input
// Provides: {"out_dir"}
// Dependencies: {}
# [doc = " The folder in which all output and intermediate artifacts should be placed."] # [doc = ""] # [doc = " This folder is inside the build directory for the package being built, and"] # [doc = " it is unique for the package in question."] # [track_caller] pub fn out_dir () -> PathBuf { to_path (var_or_panic ("OUT_DIR")) }
};
}
