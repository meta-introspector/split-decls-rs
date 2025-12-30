// Generated macro for impl_712 (impl)
macro_rules! Depcrate_collections_btree_setimpl_712 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_712"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord , A : Allocator + Clone > Ord for BTreeSet < T , A > { fn cmp (& self , other : & BTreeSet < T , A >) -> Ordering { self . map . cmp (& other . map) } }
};
}
