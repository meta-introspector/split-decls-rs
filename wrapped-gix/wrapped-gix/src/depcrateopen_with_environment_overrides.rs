// Generated macro for open_with_environment_overrides (function)
macro_rules! Depcrateopen_with_environment_overrides {
() => {
// Module: crate
// Provides: {"open_with_environment_overrides"}
// Dependencies: {}
# [doc = " Try to open a git repository directly from the environment."] # [doc = ""] # [doc = " See [`ThreadSafeRepository::open_with_environment_overrides()`]."] # [allow (clippy :: result_large_err)] pub fn open_with_environment_overrides (directory : impl Into < std :: path :: PathBuf >) -> Result < Repository , open :: Error > { ThreadSafeRepository :: open_with_environment_overrides (directory , Default :: default ()) . map (Into :: into) }
};
}
