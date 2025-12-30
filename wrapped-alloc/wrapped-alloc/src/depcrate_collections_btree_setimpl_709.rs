// Generated macro for impl_709 (impl)
macro_rules! Depcrate_collections_btree_setimpl_709 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_709"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : PartialEq , A : Allocator + Clone > PartialEq for BTreeSet < T , A > { fn eq (& self , other : & BTreeSet < T , A >) -> bool { self . map . eq (& other . map) } }
};
}
