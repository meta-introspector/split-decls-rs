// Generated macro for Union (struct)
macro_rules! Depcrate_collections_btree_setUnion {
() => {
// Module: crate::collections::btree::set
// Provides: {"Union"}
// Dependencies: {}
# [doc = " A lazy iterator producing elements in the union of `BTreeSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`union`] method on [`BTreeSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`union`]: BTreeSet::union"] # [must_use = "this returns the union as an iterator, \
              without modifying either input set"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Union < 'a , T : 'a > (MergeIterInner < Iter < 'a , T > >) ;
};
}
