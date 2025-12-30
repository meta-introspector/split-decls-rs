// Generated macro for Range (struct)
macro_rules! Depcrate_collections_btree_setRange {
() => {
// Module: crate::collections::btree::set
// Provides: {"Range"}
// Dependencies: {}
# [doc = " An iterator over a sub-range of items in a `BTreeSet`."] # [doc = ""] # [doc = " This `struct` is created by the [`range`] method on [`BTreeSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`range`]: BTreeSet::range"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [derive (Debug)] # [stable (feature = "btree_range" , since = "1.17.0")] pub struct Range < 'a , T : 'a > { iter : super :: map :: Range < 'a , T , SetValZST > , }
};
}
