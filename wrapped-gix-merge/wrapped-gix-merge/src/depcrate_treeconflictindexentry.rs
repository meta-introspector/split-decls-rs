// Generated macro for ConflictIndexEntry (struct)
macro_rules! Depcrate_treeConflictIndexEntry {
() => {
// Module: crate::tree
// Provides: {"ConflictIndexEntry"}
// Dependencies: {}
# [doc = " A conflicting entry for insertion into the index."] # [doc = " It will always be either on stage 1 (ancestor/base), 2 (ours) or 3 (theirs)"] # [derive (Debug , Clone , Copy)] pub struct ConflictIndexEntry { # [doc = " The kind of object at this stage."] # [doc = " Note that it's possible that this is a directory, for instance if a directory was replaced with a file."] pub mode : gix_object :: tree :: EntryMode , # [doc = " The id defining the state of the object."] pub id : gix_hash :: ObjectId , # [doc = " Hidden, maybe one day we can do without?"] path_hint : Option < ConflictIndexEntryPathHint > , }
};
}
