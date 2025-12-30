// Generated macro for impl_855 (impl)
macro_rules! Depcrate_collections_linked_listimpl_855 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_855"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < T > Default for IntoIter < T > { # [doc = " Creates an empty `linked_list::IntoIter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::linked_list;"] # [doc = " let iter: linked_list::IntoIter<u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { LinkedList :: new () . into_iter () } }
};
}
