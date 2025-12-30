// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_btree_setIter {
() => {
// Module: crate::collections::btree::set
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the items of a `BTreeSet`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`BTreeSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`iter`]: BTreeSet::iter"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Iter < 'a , T : 'a > { iter : Keys < 'a , T , SetValZST > , }
};
}
