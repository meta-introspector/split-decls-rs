// Generated macro for Error (enum)
macro_rules! Depcrate_commitError {
() => {
// Module: crate::commit
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`commit()`](crate::commit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to obtain the merge base between the two commits to be merged")] MergeBase (# [from] gix_revision :: merge_base :: Error) , # [error (transparent)] VirtualMergeBase (# [from] virtual_merge_base :: Error) , # [error (transparent)] MergeTree (# [from] crate :: tree :: Error) , # [error ("No common ancestor between {our_commit_id} and {their_commit_id}")] NoMergeBase { # [doc = " The commit on our side that was to be merged."] our_commit_id : gix_hash :: ObjectId , # [doc = " The commit on their side that was to be merged."] their_commit_id : gix_hash :: ObjectId , } , # [error ("Could not find ancestor, our or their commit to extract tree from")] FindCommit (# [from] gix_object :: find :: existing_object :: Error) , }
};
}
