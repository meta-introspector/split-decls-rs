// Generated macro for reorient (function)
macro_rules! Depcrate_fs_feature_gitreorient {
() => {
// Module: crate::fs::feature::git
// Provides: {"reorient"}
// Dependencies: {}
# [cfg (windows)] fn reorient (path : & Path) -> PathBuf { let unc_path = path . canonicalize () . unwrap_or_else (| _ | path . to_path_buf ()) ; let normal_path = unc_path . as_os_str () . to_str () . unwrap () . trim_start_matches ("\\\\?\\") ; PathBuf :: from (normal_path) }
};
}
