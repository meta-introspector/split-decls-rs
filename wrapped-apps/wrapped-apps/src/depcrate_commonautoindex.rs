// Generated macro for autoindex (function)
macro_rules! Depcrate_commonautoindex {
() => {
// Module: crate::common
// Provides: {"autoindex"}
// Dependencies: {}
fn autoindex (path : path :: PathBuf , index : & str) -> path :: PathBuf { if let Some (path_str) = path . to_str () { if path_str . ends_with ('/') { let path_str = format ! ("{path_str}{index}") ; return path :: PathBuf :: from (& path_str) ; } } path }
};
}
