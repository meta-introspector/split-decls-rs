// Generated macro for impl_350 (impl)
macro_rules! Depcrate_collections_btree_dedup_sorted_iterimpl_350 {
() => {
// Module: crate::collections::btree::dedup_sorted_iter
// Provides: {"impl_350"}
// Dependencies: {}
impl < K , V , I > Iterator for DedupSortedIter < K , V , I > where K : Eq , I : Iterator < Item = (K , V) > , { type Item = (K , V) ; fn next (& mut self) -> Option < (K , V) > { loop { let next = match self . iter . next () { Some (next) => next , None => return None , } ; let peeked = match self . iter . peek () { Some (peeked) => peeked , None => return Some (next) , } ; if next . 0 != peeked . 0 { return Some (next) ; } } } }
};
}
