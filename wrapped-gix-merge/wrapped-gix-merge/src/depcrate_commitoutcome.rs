// Generated macro for Outcome (struct)
macro_rules! Depcrate_commitOutcome {
() => {
// Module: crate::commit
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The result of [`commit()`](crate::commit())."] # [derive (Clone)] pub struct Outcome < 'a > { # [doc = " The outcome of the actual tree-merge."] pub tree_merge : crate :: tree :: Outcome < 'a > , # [doc = " The tree id of the base commit we used. This is either…"] # [doc = " * the single merge-base we found"] # [doc = " * the first of multiple merge-bases if [`use_first_merge_base`](Options::use_first_merge_base) was `true`."] # [doc = " * the merged tree of all merge-bases, which then isn't linked to an actual commit."] # [doc = " * an empty tree, if [`allow_missing_merge_base`](Options::allow_missing_merge_base) is enabled."] pub merge_base_tree_id : gix_hash :: ObjectId , # [doc = " The object ids of all the commits which were found to be merge-bases, or `None` if there was no merge-base."] pub merge_bases : Option < Vec < gix_hash :: ObjectId > > , # [doc = " A list of virtual commits that were created to merge multiple merge-bases into one, the last one being"] # [doc = " the one we used as merge-base for the merge."] # [doc = " As they are not reachable by anything they will be garbage collected, but knowing them provides options."] # [doc = " Would be empty if no virtual commit was needed at all as there was only a single merge-base."] # [doc = " Otherwise, the last commit id is the one with the `merge_base_tree_id`."] pub virtual_merge_bases : Vec < gix_hash :: ObjectId > , }
};
}
