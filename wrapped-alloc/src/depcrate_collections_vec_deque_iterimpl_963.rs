// Generated macro for impl_963 (impl)
macro_rules! Depcrate_collections_vec_deque_iterimpl_963 {
() => {
// Module: crate::collections::vec_deque::iter
// Provides: {"impl_963"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > ExactSizeIterator for Iter < '_ , T > { fn len (& self) -> usize { self . i1 . len () + self . i2 . len () } fn is_empty (& self) -> bool { self . i1 . is_empty () && self . i2 . is_empty () } }
};
}
