// Generated macro for ItemSliceSync (struct)
macro_rules! Depcrate_cache_delta_traverse_utilItemSliceSync {
() => {
// Module: crate::cache::delta::traverse::util
// Provides: {"ItemSliceSync"}
// Dependencies: {}
# [doc = " SAFETY: This type is used to allow access to a size-optimized vec of items that form a"] # [doc = " tree, and we need to access it concurrently with each thread taking its own root node,"] # [doc = " and working its way through all the reachable leaves."] # [doc = ""] # [doc = " The tree was built by decoding a pack whose entries refer to its bases only by OFS_DELTA -"] # [doc = " they are pointing backwards only which assures bases have to be listed first, and that each entry"] # [doc = " only has a single parent."] # [doc = ""] # [doc = " REF_DELTA entries aren't supported here, and cause immediate failure - they are expected to have"] # [doc = " been resolved before as part of the thin-pack handling."] # [doc = ""] # [doc = " If we somehow would allow REF_DELTA entries to point to an in-pack object, then in theory malicious packs could"] # [doc = " cause all kinds of graphs as they can point anywhere in the pack, but they still can't link an entry to"] # [doc = " more than one base. And that's what one would really have to do for two threads to encounter the same child."] # [doc = ""] # [doc = " Thus I believe it's impossible for this data structure to end up in a place where it violates its assumption."] pub (super) struct ItemSliceSync < 'a , T > where T : Send , { items : * mut T , # [cfg (debug_assertions)] len : usize , phantom : PhantomData < & 'a mut T > , }
};
}
