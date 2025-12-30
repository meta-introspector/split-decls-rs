// Generated macro for set_global_root (function)
macro_rules! Depcrate_pathsset_global_root {
() => {
// Module: crate::paths
// Provides: {"set_global_root"}
// Dependencies: {}
fn set_global_root (tmp_dir : & 'static str) { let mut lock = GLOBAL_ROOT . get_or_init (| | Default :: default ()) . lock () . unwrap () ; if lock . is_none () { let mut root = PathBuf :: from (tmp_dir) ; root . push (CARGO_INTEGRATION_TEST_DIR) ; * lock = Some (root) ; } }
};
}
