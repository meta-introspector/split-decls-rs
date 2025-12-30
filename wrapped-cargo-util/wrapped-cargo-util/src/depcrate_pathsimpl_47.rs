// Generated macro for impl_47 (impl)
macro_rules! Depcrate_pathsimpl_47 {
() => {
// Module: crate::paths
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > PathAncestors < 'a > { fn new (path : & 'a Path , stop_root_at : Option < & Path >) -> PathAncestors < 'a > { let stop_at = env :: var ("__CARGO_TEST_ROOT") . ok () . map (PathBuf :: from) . or_else (| | stop_root_at . map (| p | p . to_path_buf ())) ; PathAncestors { current : Some (path) , stop_at , } } }
};
}
