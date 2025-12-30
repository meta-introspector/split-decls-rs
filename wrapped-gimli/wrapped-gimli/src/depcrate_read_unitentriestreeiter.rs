// Generated macro for EntriesTreeIter (struct)
macro_rules! Depcrate_read_unitEntriesTreeIter {
() => {
// Module: crate::read::unit
// Provides: {"EntriesTreeIter"}
// Dependencies: {}
# [doc = " An iterator that allows traversal of the children of an"] # [doc = " `EntriesTreeNode`."] # [doc = ""] # [doc = " The items returned by this iterator are also `EntriesTreeNode`s,"] # [doc = " which allow recursive traversal of grandchildren, etc."] # [derive (Debug)] pub struct EntriesTreeIter < 'abbrev , 'unit , 'tree , R : Reader > { tree : & 'tree mut EntriesTree < 'abbrev , 'unit , R > , depth : isize , empty : bool , }
};
}
