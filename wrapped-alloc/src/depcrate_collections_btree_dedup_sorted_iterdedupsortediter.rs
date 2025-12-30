// Generated macro for DedupSortedIter (struct)
macro_rules! Depcrate_collections_btree_dedup_sorted_iterDedupSortedIter {
() => {
// Module: crate::collections::btree::dedup_sorted_iter
// Provides: {"DedupSortedIter"}
// Dependencies: {}
# [doc = " An iterator for deduping the key of a sorted iterator."] # [doc = " When encountering the duplicated key, only the last key-value pair is yielded."] # [doc = ""] # [doc = " Used by [`BTreeMap::bulk_build_from_sorted_iter`][1]."] # [doc = ""] # [doc = " [1]: crate::collections::BTreeMap::bulk_build_from_sorted_iter"] pub (super) struct DedupSortedIter < K , V , I > where I : Iterator < Item = (K , V) > , { iter : Peekable < I > , }
};
}
