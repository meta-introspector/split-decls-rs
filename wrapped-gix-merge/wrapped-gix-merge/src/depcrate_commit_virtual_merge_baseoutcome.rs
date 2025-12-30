// Generated macro for Outcome (struct)
macro_rules! Depcrate_commit_virtual_merge_baseOutcome {
() => {
// Module: crate::commit::virtual_merge_base
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome produced by [`commit::merge_base()`](crate::commit::virtual_merge_base())."] pub struct Outcome { # [doc = " The commit ids of all the virtual merge bases we have produced in the process of recursively merging the merge-bases."] # [doc = " As they have been written to the object database, they are still available until they are garbage collected."] # [doc = " The last one is the most recently produced and the one returned as `commit_id`."] # [doc = " This is never empty."] pub virtual_merge_bases : Vec < gix_hash :: ObjectId > , # [doc = " The id of the commit that was created to hold the merged tree."] pub commit_id : gix_hash :: ObjectId , # [doc = " The hash of the merged tree."] pub tree_id : gix_hash :: ObjectId , }
};
}
