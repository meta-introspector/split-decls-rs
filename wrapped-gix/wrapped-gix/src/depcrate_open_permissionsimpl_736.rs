// Generated macro for impl_736 (impl)
macro_rules! Depcrate_open_permissionsimpl_736 {
() => {
// Module: crate::open::permissions
// Provides: {"impl_736"}
// Dependencies: {}
impl Environment { # [doc = " Allow access to the entire environment."] pub fn all () -> Self { let allow = gix_sec :: Permission :: Allow ; Environment { xdg_config_home : allow , home : allow , git_prefix : allow , ssh_prefix : allow , http_transport : allow , identity : allow , objects : allow , } } # [doc = " Don't allow loading any environment variables."] pub fn isolated () -> Self { let deny = gix_sec :: Permission :: Deny ; Environment { xdg_config_home : deny , home : deny , ssh_prefix : deny , git_prefix : deny , http_transport : deny , identity : deny , objects : deny , } } }
};
}
