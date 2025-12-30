// Generated macro for rustc_linker (function)
macro_rules! Depcrate_inputrustc_linker {
() => {
// Module: crate::input
// Provides: {"rustc_linker"}
// Dependencies: {}
# [doc = " The linker that Cargo has resolved to use for the current target, if specified."] # [doc = ""] # [doc = " [`target.*.linker`]: https://doc.rust-lang.org/stable/cargo/reference/config.html#targettriplelinker"] # [track_caller] pub fn rustc_linker () -> Option < PathBuf > { ENV . get ("RUSTC_LINKER") . map (to_path) }
};
}
