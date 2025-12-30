// Generated macro for IntoIter (struct)
macro_rules! Depcrate_collections_btree_setIntoIter {
() => {
// Module: crate::collections::btree::set
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the items of a `BTreeSet` in ascending order."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`BTreeSet`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: BTreeSet#method.into_iter"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct IntoIter < T , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { iter : super :: map :: IntoIter < T , SetValZST , A > , }
};
}
