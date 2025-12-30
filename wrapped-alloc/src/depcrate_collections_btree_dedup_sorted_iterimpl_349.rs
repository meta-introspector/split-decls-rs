// Generated macro for impl_349 (impl)
macro_rules! Depcrate_collections_btree_dedup_sorted_iterimpl_349 {
() => {
// Module: crate::collections::btree::dedup_sorted_iter
// Provides: {"impl_349"}
// Dependencies: {}
impl < K , V , I > DedupSortedIter < K , V , I > where I : Iterator < Item = (K , V) > , { pub (super) fn new (iter : I) -> Self { Self { iter : iter . peekable () } } }
};
}
