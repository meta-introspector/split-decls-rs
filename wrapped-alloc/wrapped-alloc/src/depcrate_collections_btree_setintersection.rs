// Generated macro for Intersection (struct)
macro_rules! Depcrate_collections_btree_setIntersection {
() => {
// Module: crate::collections::btree::set
// Provides: {"Intersection"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the intersection of `BTreeSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`intersection`] method on [`BTreeSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`intersection`]: BTreeSet::intersection"] # [must_use = "this returns the intersection as an iterator, \
              without modifying either input set"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Intersection < 'a , T : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { inner : IntersectionInner < 'a , T , A > , }
};
}
