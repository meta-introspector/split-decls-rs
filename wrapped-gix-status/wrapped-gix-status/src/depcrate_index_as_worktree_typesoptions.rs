// Generated macro for Options (struct)
macro_rules! Depcrate_index_as_worktree_typesOptions {
() => {
// Module: crate::index_as_worktree::types
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options that control how the index status with a worktree is computed."] # [derive (Clone , Default , Debug , PartialEq , Eq , Hash)] pub struct Options { # [doc = " Capabilities of the file system which affect the status computation."] pub fs : gix_fs :: Capabilities , # [doc = " If set, don't use more than this amount of threads."] # [doc = " Otherwise, usually use as many threads as there are logical cores."] # [doc = " A value of 0 is interpreted as no-limit"] pub thread_limit : Option < usize > , # [doc = " Options that control how stat comparisons are made when checking if a file is fresh."] pub stat : gix_index :: entry :: stat :: Options , }
};
}
