// Generated macro for impl_736 (impl)
macro_rules! Depcrate_collections_btree_setimpl_736 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_736"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator + Clone > IntoIterator for BTreeSet < T , A > { type Item = T ; type IntoIter = IntoIter < T , A > ; # [doc = " Gets an iterator for moving out the `BTreeSet`'s contents in ascending order."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BTreeSet;"] # [doc = ""] # [doc = " let set = BTreeSet::from([1, 2, 3, 4]);"] # [doc = ""] # [doc = " let v: Vec<_> = set.into_iter().collect();"] # [doc = " assert_eq!(v, [1, 2, 3, 4]);"] # [doc = " ```"] fn into_iter (self) -> IntoIter < T , A > { IntoIter { iter : self . map . into_iter () } } }
};
}
