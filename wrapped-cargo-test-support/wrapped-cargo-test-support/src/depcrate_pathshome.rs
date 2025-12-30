// Generated macro for home (function)
macro_rules! Depcrate_pathshome {
() => {
// Module: crate::paths
// Provides: {"home"}
// Dependencies: {}
# [doc = " Path to the current test's `$HOME`"] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0/home`"] pub fn home () -> PathBuf { let mut path = root () ; path . push ("home") ; path . mkdir_p () ; path }
};
}
