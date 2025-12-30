// Generated macro for Difference (struct)
macro_rules! Depcrate_collections_btree_setDifference {
() => {
// Module: crate::collections::btree::set
// Provides: {"Difference"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the difference of `BTreeSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`difference`] method on [`BTreeSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`difference`]: BTreeSet::difference"] # [must_use = "this returns the difference as an iterator, \
              without modifying either input set"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Difference < 'a , T : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { inner : DifferenceInner < 'a , T , A > , }
};
}
