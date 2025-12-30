// Generated macro for Options (struct)
macro_rules! Depcrate_diffOptions {
() => {
// Module: crate::diff
// Provides: {"Options"}
// Dependencies: {}
# [doc = " General diff-related options for configuring rename-tracking and blob diffs."] # [derive (Debug , Copy , Clone)] pub struct Options { location : Option < Location > , # [cfg (feature = "blob-diff")] rewrites : Option < gix_diff :: Rewrites > , }
};
}
