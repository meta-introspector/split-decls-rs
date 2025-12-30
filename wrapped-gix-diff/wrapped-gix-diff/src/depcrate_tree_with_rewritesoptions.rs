// Generated macro for Options (struct)
macro_rules! Depcrate_tree_with_rewritesOptions {
() => {
// Module: crate::tree_with_rewrites
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for use in [`tree_with_rewrites()`](super::tree_with_rewrites())."] # [derive (Default , Clone , Debug)] pub struct Options { # [doc = " Determine how locations of changes, i.e. their repository-relative path, should be tracked."] # [doc = " If `None`, locations will always be empty."] pub location : Option < Location > , # [doc = " If not `None`, rename tracking will be performed accordingly."] pub rewrites : Option < Rewrites > , }
};
}
