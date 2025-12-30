// Generated macro for ConflictIndexEntryPathHint (enum)
macro_rules! Depcrate_treeConflictIndexEntryPathHint {
() => {
// Module: crate::tree
// Provides: {"ConflictIndexEntryPathHint"}
// Dependencies: {}
# [doc = " A hint for [`apply_index_entries()`] to know which paths to use for an entry."] # [doc = " This is only used when necessary."] # [derive (Debug , Clone , Copy)] enum ConflictIndexEntryPathHint { # [doc = " Use the previous path, i.e. rename source."] Source , # [doc = " Use the current path as it is in the tree."] Current , # [doc = " Use the path of the final destination, or *their* name."] # [doc = " It's definitely finicky, as we don't store the actual path and instead refer to it."] RenamedOrTheirs , }
};
}
