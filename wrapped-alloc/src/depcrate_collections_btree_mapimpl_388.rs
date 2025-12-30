// Generated macro for impl_388 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_388 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_388"}
// Dependencies: {}
# [stable (feature = "btree_drop" , since = "1.7.0")] unsafe impl < # [may_dangle] K , # [may_dangle] V , A : Allocator + Clone > Drop for BTreeMap < K , V , A > { fn drop (& mut self) { drop (unsafe { ptr :: read (self) } . into_iter ()) } }
};
}
