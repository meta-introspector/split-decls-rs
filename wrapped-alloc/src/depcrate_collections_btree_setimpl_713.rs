// Generated macro for impl_713 (impl)
macro_rules! Depcrate_collections_btree_setimpl_713 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_713"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Clone , A : Allocator + Clone > Clone for BTreeSet < T , A > { fn clone (& self) -> Self { BTreeSet { map : self . map . clone () } } fn clone_from (& mut self , source : & Self) { self . map . clone_from (& source . map) ; } }
};
}
