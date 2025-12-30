// Generated macro for EntriesTreeNode (struct)
macro_rules! Depcrate_read_unitEntriesTreeNode {
() => {
// Module: crate::read::unit
// Provides: {"EntriesTreeNode"}
// Dependencies: {}
# [doc = " A node in the Debugging Information Entry tree."] # [doc = ""] # [doc = " The root node of a tree can be obtained"] # [doc = " via [`EntriesTree::root`](./struct.EntriesTree.html#method.root)."] # [derive (Debug)] pub struct EntriesTreeNode < 'abbrev , 'unit , 'tree , R : Reader > { tree : & 'tree mut EntriesTree < 'abbrev , 'unit , R > , depth : isize , }
};
}
