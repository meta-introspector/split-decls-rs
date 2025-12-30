// Generated macro for index_entry_at_path (function)
macro_rules! Depcrate_tree_functionindex_entry_at_path {
() => {
// Module: crate::tree::function
// Provides: {"index_entry_at_path"}
// Dependencies: {}
fn index_entry_at_path (mode : & gix_object :: tree :: EntryMode , id : & gix_hash :: ObjectId , hint : ConflictIndexEntryPathHint ,) -> Option < ConflictIndexEntry > { Some (ConflictIndexEntry { mode : * mode , id : * id , path_hint : Some (hint) , }) }
};
}
