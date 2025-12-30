// Generated macro for cargo_home (function)
macro_rules! Depcrate_pathscargo_home {
() => {
// Module: crate::paths
// Provides: {"cargo_home"}
// Dependencies: {}
# [doc = " Path to the current test's `$CARGO_HOME`"] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/home/.cargo`"] pub fn cargo_home () -> PathBuf { home () . join (".cargo") }
};
}
