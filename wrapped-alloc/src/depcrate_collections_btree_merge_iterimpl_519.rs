// Generated macro for impl_519 (impl)
macro_rules! Depcrate_collections_btree_merge_iterimpl_519 {
() => {
// Module: crate::collections::btree::merge_iter
// Provides: {"impl_519"}
// Dependencies: {}
impl < I : Iterator > Clone for MergeIterInner < I > where I : Clone , I :: Item : Clone , { fn clone (& self) -> Self { Self { a : self . a . clone () , b : self . b . clone () , peeked : self . peeked . clone () } } }
};
}
