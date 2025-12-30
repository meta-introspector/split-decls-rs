// Generated macro for discover_with_environment_overrides (function)
macro_rules! Depcratediscover_with_environment_overrides {
() => {
// Module: crate
// Provides: {"discover_with_environment_overrides"}
// Dependencies: {}
# [doc = " Try to discover a git repository directly from the environment."] # [doc = ""] # [doc = " For details, see [`ThreadSafeRepository::discover_with_environment_overrides_opts()`]."] # [allow (clippy :: result_large_err)] pub fn discover_with_environment_overrides (directory : impl AsRef < std :: path :: Path > ,) -> Result < Repository , discover :: Error > { ThreadSafeRepository :: discover_with_environment_overrides (directory) . map (Into :: into) }
};
}
