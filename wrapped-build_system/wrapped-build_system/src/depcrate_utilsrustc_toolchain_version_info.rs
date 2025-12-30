// Generated macro for rustc_toolchain_version_info (function)
macro_rules! Depcrate_utilsrustc_toolchain_version_info {
() => {
// Module: crate::utils
// Provides: {"rustc_toolchain_version_info"}
// Dependencies: {}
pub fn rustc_toolchain_version_info (toolchain : & str) -> Result < RustcVersionInfo , String > { rustc_version_info_inner (None , Some (toolchain)) }
};
}
