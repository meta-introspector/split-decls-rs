// Generated macro for impl_708 (impl)
macro_rules! Depcrate_collections_btree_setimpl_708 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_708"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Hash , A : Allocator + Clone > Hash for BTreeSet < T , A > { fn hash < H : Hasher > (& self , state : & mut H) { self . map . hash (state) } }
};
}
