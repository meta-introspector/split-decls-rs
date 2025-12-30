// Generated macro for rustc_wrapper (function)
macro_rules! Depcrate_inputrustc_wrapper {
() => {
// Module: crate::input
// Provides: {"rustc_wrapper"}
// Dependencies: {}
# [doc = " The rustc wrapper, if any, that Cargo is using. See [`build.rustc-wrapper`]."] # [doc = ""] # [doc = " [`build.rustc-wrapper`]: https://doc.rust-lang.org/stable/cargo/reference/config.html#buildrustc-wrapper"] # [track_caller] pub fn rustc_wrapper () -> Option < PathBuf > { ENV . get ("RUSTC_WRAPPER") . map (to_path) }
};
}
