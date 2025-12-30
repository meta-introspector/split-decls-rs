// Generated macro for submodules_ (function)
macro_rules! Depcrate_core_config_configsubmodules_ {
() => {
// Module: crate::core::config::config
// Provides: {"submodules_"}
// Dependencies: {}
pub fn submodules_ (submodules : & Option < bool > , rust_info : & channel :: GitInfo) -> bool { submodules . unwrap_or (rust_info . is_managed_git_subrepository ()) }
};
}
