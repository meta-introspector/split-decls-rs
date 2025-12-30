// Generated macro for cwd (function)
macro_rules! Depcrate_path_helperscwd {
() => {
// Module: crate::path_helpers
// Provides: {"cwd"}
// Dependencies: {}
# [doc = " Return the current working directory."] # [doc = ""] # [doc = " This forwards to [`std::env::current_dir`], please see its docs regarding platform-specific"] # [doc = " behavior."] # [must_use] pub fn cwd () -> PathBuf { std :: env :: current_dir () . unwrap () }
};
}
