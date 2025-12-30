// Generated macro for rustc (function)
macro_rules! Depcrate_inputrustc {
() => {
// Module: crate::input
// Provides: {"rustc"}
// Dependencies: {}
# [doc = " The compiler that Cargo has resolved to use."] # [track_caller] pub fn rustc () -> PathBuf { to_path (var_or_panic ("RUSTC")) }
};
}
