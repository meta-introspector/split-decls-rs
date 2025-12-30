// Generated macro for impl_928 (impl)
macro_rules! Depcrate_collections_vec_deque_iter_mutimpl_928 {
() => {
// Module: crate::collections::vec_deque::iter_mut
// Provides: {"impl_928"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > ExactSizeIterator for IterMut < '_ , T > { fn len (& self) -> usize { self . i1 . len () + self . i2 . len () } fn is_empty (& self) -> bool { self . i1 . is_empty () && self . i2 . is_empty () } }
};
}
