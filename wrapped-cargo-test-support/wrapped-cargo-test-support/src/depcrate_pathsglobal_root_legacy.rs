// Generated macro for global_root_legacy (function)
macro_rules! Depcrate_pathsglobal_root_legacy {
() => {
// Module: crate::paths
// Provides: {"global_root_legacy"}
// Dependencies: {}
# [doc = " This is used when running cargo is pre-CARGO_TARGET_TMPDIR"] # [doc = " TODO: Remove when `CARGO_TARGET_TMPDIR` grows old enough."] fn global_root_legacy () -> PathBuf { let mut path = t ! (env :: current_exe ()) ; path . pop () ; path . pop () ; path . push ("tmp") ; path . mkdir_p () ; path }
};
}
