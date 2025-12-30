// Generated macro for impl_24 (impl)
macro_rules! Depcrate_upwards_typesimpl_24 {
() => {
// Module: crate::upwards::types
// Provides: {"impl_24"}
// Dependencies: {}
impl Options < '_ > { # [doc = " Loads discovery options overrides from the environment."] # [doc = ""] # [doc = " The environment variables are:"] # [doc = " - `GIT_CEILING_DIRECTORIES` for `ceiling_dirs`"] # [doc = ""] # [doc = " Note that `GIT_DISCOVERY_ACROSS_FILESYSTEM` for `cross_fs` is **not** read,"] # [doc = " as it requires parsing of `git-config` style boolean values."] pub fn apply_environment (mut self) -> Self { let name = "GIT_CEILING_DIRECTORIES" ; if let Some (ceiling_dirs) = env :: var_os (name) { self . ceiling_dirs = parse_ceiling_dirs (& ceiling_dirs) ; } self } }
};
}
