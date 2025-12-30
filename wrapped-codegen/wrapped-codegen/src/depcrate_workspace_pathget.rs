// Generated macro for get (function)
macro_rules! Depcrate_workspace_pathget {
() => {
// Module: crate::workspace_path
// Provides: {"get"}
// Dependencies: {}
pub fn get (relative_to_workspace_root : impl AsRef < Path >) -> PathBuf { let mut path = PathBuf :: from (env ! ("CARGO_MANIFEST_DIR")) ; assert ! (path . pop ()) ; path . push (relative_to_workspace_root) ; path }
};
}
