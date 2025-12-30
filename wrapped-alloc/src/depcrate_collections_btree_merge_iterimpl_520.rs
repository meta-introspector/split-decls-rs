// Generated macro for impl_520 (impl)
macro_rules! Depcrate_collections_btree_merge_iterimpl_520 {
() => {
// Module: crate::collections::btree::merge_iter
// Provides: {"impl_520"}
// Dependencies: {}
impl < I : Iterator > Debug for MergeIterInner < I > where I : Debug , I :: Item : Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("MergeIterInner") . field (& self . a) . field (& self . b) . field (& self . peeked) . finish () } }
};
}
