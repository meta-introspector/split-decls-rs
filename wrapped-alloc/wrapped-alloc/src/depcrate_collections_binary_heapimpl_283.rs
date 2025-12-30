// Generated macro for impl_283 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_283 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_283"}
// Dependencies: {}
# [stable (feature = "default_iters_sequel" , since = "1.82.0")] impl < T > Default for Iter < '_ , T > { # [doc = " Creates an empty `binary_heap::Iter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::binary_heap;"] # [doc = " let iter: binary_heap::Iter<'_, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { Iter { iter : Default :: default () } } }
};
}
