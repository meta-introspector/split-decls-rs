// Generated macro for Error (enum)
macro_rules! Depcrate_commit_virtual_merge_baseError {
() => {
// Module: crate::commit::virtual_merge_base
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`commit::merge_base()`](crate::commit::virtual_merge_base())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] MergeTree (# [from] crate :: tree :: Error) , # [error ("Failed to write tree for merged merge-base or virtual commit")] WriteObject (gix_object :: write :: Error) , # [error ("Failed to decode a commit needed to build a virtual merge-base")] DecodeCommit (# [from] gix_object :: decode :: Error) , # [error ("Conflicts occurred when trying to resolve multiple merge-bases by merging them. This is most certainly a bug.")] VirtualMergeBaseConflict , # [error ("Could not find commit to use as basis for a virtual commit")] FindCommit (# [from] gix_object :: find :: existing_object :: Error) , }
};
}
