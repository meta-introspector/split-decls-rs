// Generated macro for SymmetricDifference (struct)
macro_rules! Depcrate_collections_btree_setSymmetricDifference {
() => {
// Module: crate::collections::btree::set
// Provides: {"SymmetricDifference"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the symmetric difference of `BTreeSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`symmetric_difference`] method on"] # [doc = " [`BTreeSet`]. See its documentation for more."] # [doc = ""] # [doc = " [`symmetric_difference`]: BTreeSet::symmetric_difference"] # [must_use = "this returns the difference as an iterator, \
              without modifying either input set"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct SymmetricDifference < 'a , T : 'a > (MergeIterInner < Iter < 'a , T > >) ;
};
}
