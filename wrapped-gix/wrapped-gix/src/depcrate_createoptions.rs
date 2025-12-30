// Generated macro for Options (struct)
macro_rules! Depcrate_createOptions {
() => {
// Module: crate::create
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for use in [`into()`];"] # [derive (Copy , Clone , Default)] pub struct Options { # [doc = " If true, and the kind of repository to create has a worktree, then the destination directory must be empty."] # [doc = ""] # [doc = " By default repos with worktree can be initialized into a non-empty repository as long as there is no `.git` directory."] pub destination_must_be_empty : bool , # [doc = " If set, use these filesystem capabilities to populate the respective git-config fields."] # [doc = " If `None`, the directory will be probed."] pub fs_capabilities : Option < gix_fs :: Capabilities > , }
};
}
