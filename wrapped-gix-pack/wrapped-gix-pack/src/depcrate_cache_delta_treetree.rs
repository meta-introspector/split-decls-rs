// Generated macro for Tree (struct)
macro_rules! Depcrate_cache_delta_treeTree {
() => {
// Module: crate::cache::delta::tree
// Provides: {"Tree"}
// Dependencies: {}
# [doc = " A tree that allows one-time iteration over all nodes and their children, consuming it in the process,"] # [doc = " while being shareable among threads without a lock."] # [doc = " It does this by making the guarantee that iteration only happens once."] pub struct Tree < T > { # [doc = " The root nodes, i.e. base objects"] root_items : Vec < Item < T > > , # [doc = " The child nodes, i.e. those that rely a base object, like ref and ofs delta objects"] child_items : Vec < Item < T > > , # [doc = " The last encountered node was either a root or a child."] last_seen : Option < NodeKind > , # [doc = " Future child offsets, associating their offset into the pack with their index in the items array."] # [doc = " (parent_offset, child_index)"] future_child_offsets : Vec < (crate :: data :: Offset , usize) > , }
};
}
