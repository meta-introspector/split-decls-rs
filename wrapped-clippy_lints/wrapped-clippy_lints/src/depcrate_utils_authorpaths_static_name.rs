// Generated macro for paths_static_name (function)
macro_rules! Depcrate_utils_authorpaths_static_name {
() => {
// Module: crate::utils::author
// Provides: {"paths_static_name"}
// Dependencies: {}
fn paths_static_name (cx : & LateContext < '_ > , id : DefId) -> String { cx . get_def_path (id) . iter () . map (Symbol :: as_str) . filter (| s | ! s . starts_with ('<')) . join ("_") . to_uppercase () }
};
}
