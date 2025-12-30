// Generated macro for impl_711 (impl)
macro_rules! Depcrate_collections_btree_setimpl_711 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_711"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : PartialOrd , A : Allocator + Clone > PartialOrd for BTreeSet < T , A > { fn partial_cmp (& self , other : & BTreeSet < T , A >) -> Option < Ordering > { self . map . partial_cmp (& other . map) } }
};
}
