// Generated macro for Options (struct)
macro_rules! Depcrate_commitOptions {
() => {
// Module: crate::commit
// Provides: {"Options"}
// Dependencies: {}
# [doc = " A way to configure [`commit()`](crate::commit())."] # [derive (Default , Debug , Clone)] pub struct Options { # [doc = " If `true`, merging unrelated commits is allowed, with the merge-base being assumed as empty tree."] pub allow_missing_merge_base : bool , # [doc = " Options to define how trees should be merged."] pub tree_merge : crate :: tree :: Options , # [doc = " If `true`, do not merge multiple merge-bases into one. Instead, just use the first one."] # [doc (alias = "no_recursive" , alias = "git2")] pub use_first_merge_base : bool , }
};
}
