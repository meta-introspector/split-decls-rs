// Generated macro for impl_960 (impl)
macro_rules! Depcrate_config_tree_sections_initimpl_960 {
() => {
// Module: crate::config::tree::sections::init
// Provides: {"impl_960"}
// Dependencies: {}
impl Init { # [doc = " The `init.defaultBranch` key."] pub const DEFAULT_BRANCH : keys :: Any = keys :: Any :: new ("defaultBranch" , & config :: Tree :: INIT) . with_deviation ("If not set, we use `main` instead of `master`") ; }
};
}
