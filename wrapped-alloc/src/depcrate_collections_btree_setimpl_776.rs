// Generated macro for impl_776 (impl)
macro_rules! Depcrate_collections_btree_setimpl_776 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_776"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T : Ord > Iterator for Union < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { let (a_next , b_next) = self . 0 . nexts (Self :: Item :: cmp) ; a_next . or (b_next) } fn size_hint (& self) -> (usize , Option < usize >) { let (a_len , b_len) = self . 0 . lens () ; (max (a_len , b_len) , Some (a_len + b_len)) } fn min (mut self) -> Option < & 'a T > { self . next () } }
};
}
