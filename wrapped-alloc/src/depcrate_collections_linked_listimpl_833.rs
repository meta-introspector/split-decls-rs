// Generated macro for impl_833 (impl)
macro_rules! Depcrate_collections_linked_listimpl_833 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_833"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < T > Default for Iter < '_ , T > { # [doc = " Creates an empty `linked_list::Iter`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::linked_list;"] # [doc = " let iter: linked_list::Iter<'_, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { Iter { head : None , tail : None , len : 0 , marker : Default :: default () } } }
};
}
