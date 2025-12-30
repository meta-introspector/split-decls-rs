// Generated macro for impl_770 (impl)
macro_rules! Depcrate_collections_btree_setimpl_770 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_770"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T : Ord > Iterator for SymmetricDifference < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { loop { let (a_next , b_next) = self . 0 . nexts (Self :: Item :: cmp) ; if a_next . and (b_next) . is_none () { return a_next . or (b_next) ; } } } fn size_hint (& self) -> (usize , Option < usize >) { let (a_len , b_len) = self . 0 . lens () ; (0 , Some (a_len + b_len)) } fn min (mut self) -> Option < & 'a T > { self . next () } }
};
}
