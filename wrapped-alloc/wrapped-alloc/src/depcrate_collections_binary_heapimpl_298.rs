// Generated macro for impl_298 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_298 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_298"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < T > Default for IntoIter < T > { # [doc = " Creates an empty `binary_heap::IntoIter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::binary_heap;"] # [doc = " let iter: binary_heap::IntoIter<u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { IntoIter { iter : Default :: default () } } }
};
}
