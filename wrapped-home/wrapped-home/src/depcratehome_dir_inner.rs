// Generated macro for home_dir_inner (function)
macro_rules! Depcratehome_dir_inner {
() => {
// Module: crate
// Provides: {"home_dir_inner"}
// Dependencies: {}
# [cfg (unix)] fn home_dir_inner () -> Option < PathBuf > { # [allow (deprecated)] std :: env :: home_dir () }
};
}
