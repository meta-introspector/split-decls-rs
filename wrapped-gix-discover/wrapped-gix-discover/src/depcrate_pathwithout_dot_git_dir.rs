// Generated macro for without_dot_git_dir (function)
macro_rules! Depcrate_pathwithout_dot_git_dir {
() => {
// Module: crate::path
// Provides: {"without_dot_git_dir"}
// Dependencies: {}
# [doc = " Conditionally pop a trailing `.git` dir if present."] pub fn without_dot_git_dir (mut path : PathBuf) -> PathBuf { if path . file_name () . and_then (std :: ffi :: OsStr :: to_str) == Some (DOT_GIT_DIR) { path . pop () ; } path }
};
}
