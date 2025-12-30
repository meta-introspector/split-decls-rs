// Generated macro for impl_337 (impl)
macro_rules! Depcrate_collections_btree_appendimpl_337 {
() => {
// Module: crate::collections::btree::append
// Provides: {"impl_337"}
// Dependencies: {}
impl < K : Ord , V , I > Iterator for MergeIter < K , V , I > where I : Iterator < Item = (K , V) > + FusedIterator , { type Item = (K , V) ; # [doc = " If two keys are equal, returns the key-value pair from the right source."] fn next (& mut self) -> Option < (K , V) > { let (a_next , b_next) = self . 0 . nexts (| a : & (K , V) , b : & (K , V) | K :: cmp (& a . 0 , & b . 0)) ; b_next . or (a_next) } }
};
}
