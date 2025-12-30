// Generated macro for ConflictIndexEntry (struct)
macro_rules! Depcrate_index_as_worktree_typesConflictIndexEntry {
() => {
// Module: crate::index_as_worktree::types
// Provides: {"ConflictIndexEntry"}
// Dependencies: {}
# [doc = " Like [`gix_index::Entry`], but without disk-metadata."] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct ConflictIndexEntry { # [doc = " The object id for this entry's ODB representation (assuming it's up-to-date with it)."] pub id : gix_hash :: ObjectId , # [doc = " Additional flags for use in algorithms and for efficiently storing stage information, primarily"] # [doc = " to obtain the [stage](entry::Flags::stage())."] pub flags : entry :: Flags , # [doc = " The kind of item this entry represents - it's not all blobs in the index anymore."] pub mode : entry :: Mode , }
};
}
