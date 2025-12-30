// Generated macro for get_sysroot_dir (function)
macro_rules! Depcrate_utilsget_sysroot_dir {
() => {
// Module: crate::utils
// Provides: {"get_sysroot_dir"}
// Dependencies: {}
pub fn get_sysroot_dir () -> PathBuf { Path :: new (crate :: BUILD_DIR) . join ("build_sysroot") }
};
}
