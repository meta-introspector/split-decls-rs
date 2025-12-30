// Generated macro for ENABLED (const)
macro_rules! Depcrate_fs_feature_xattrENABLED {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"ENABLED"}
// Dependencies: {}
pub const ENABLED : bool = cfg ! (any (target_os = "macos" , target_os = "linux" , target_os = "netbsd" , target_os = "freebsd")) ;
};
}
