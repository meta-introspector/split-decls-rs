// Generated macro for impl_432 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_432 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_432"}
// Dependencies: {}
# [stable (feature = "btree_drop" , since = "1.7.0")] impl < K , V , A : Allocator + Clone > Drop for IntoIter < K , V , A > { fn drop (& mut self) { struct DropGuard < 'a , K , V , A : Allocator + Clone > (& 'a mut IntoIter < K , V , A >) ; impl < 'a , K , V , A : Allocator + Clone > Drop for DropGuard < 'a , K , V , A > { fn drop (& mut self) { while let Some (kv) = self . 0 . dying_next () { unsafe { kv . drop_key_val () } ; } } } while let Some (kv) = self . dying_next () { let guard = DropGuard (self) ; unsafe { kv . drop_key_val () } ; mem :: forget (guard) ; } } }
};
}
