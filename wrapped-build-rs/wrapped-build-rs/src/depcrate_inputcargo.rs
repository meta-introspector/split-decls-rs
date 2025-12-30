// Generated macro for cargo (function)
macro_rules! Depcrate_inputcargo {
() => {
// Module: crate::input
// Provides: {"cargo"}
// Dependencies: {}
# [doc = " Path to the `cargo` binary performing the build."] # [track_caller] pub fn cargo () -> PathBuf { to_path (var_or_panic ("CARGO")) }
};
}
