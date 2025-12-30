// Generated macro for Item (struct)
macro_rules! Depcrate_cache_delta_treeItem {
() => {
// Module: crate::cache::delta::tree
// Provides: {"Item"}
// Dependencies: {}
# [doc = " An item stored within the [`Tree`] whose data is stored in a pack file, identified by"] # [doc = " the offset of its first (`offset`) and last (`next_offset`) bytes."] # [doc = ""] # [doc = " It represents either a root entry, or one that relies on a base to be resolvable,"] # [doc = " alongside associated `data` `T`."] pub struct Item < T > { # [doc = " The offset into the pack file at which the pack entry's data is located."] pub offset : crate :: data :: Offset , # [doc = " The offset of the next item in the pack file."] pub next_offset : crate :: data :: Offset , # [doc = " Data to store with each Item, effectively data associated with each entry in a pack."] pub data : T , # [doc = " Indices into our Tree's `items`, one for each pack entry that depends on us."] # [doc = ""] # [doc = " Limited to u32 as that's the maximum amount of objects in a pack."] children : Vec < u32 > , }
};
}
