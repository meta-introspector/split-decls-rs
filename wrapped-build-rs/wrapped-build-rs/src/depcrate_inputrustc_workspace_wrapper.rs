// Generated macro for rustc_workspace_wrapper (function)
macro_rules! Depcrate_inputrustc_workspace_wrapper {
() => {
// Module: crate::input
// Provides: {"rustc_workspace_wrapper"}
// Dependencies: {}
# [doc = " The rustc wrapper, if any, that Cargo is using for workspace members. See"] # [doc = " [`build.rustc-workspace-wrapper`]."] # [doc = ""] # [doc = " [`build.rustc-workspace-wrapper`]: https://doc.rust-lang.org/stable/cargo/reference/config.html#buildrustc-workspace-wrapper"] # [track_caller] pub fn rustc_workspace_wrapper () -> Option < PathBuf > { ENV . get ("RUSTC_WORKSPACE_WRAPPER") . map (to_path) }
};
}
