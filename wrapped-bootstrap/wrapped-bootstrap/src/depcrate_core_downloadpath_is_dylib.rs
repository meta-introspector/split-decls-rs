// Generated macro for path_is_dylib (function)
macro_rules! Depcrate_core_downloadpath_is_dylib {
() => {
// Module: crate::core::download
// Provides: {"path_is_dylib"}
// Dependencies: {}
fn path_is_dylib (path : & Path) -> bool { path . to_str () . is_some_and (| path | path . contains (".so")) }
};
}
