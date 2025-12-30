// Generated macro for IntoIter (struct)
macro_rules! Depcrate_collections_btree_mapIntoIter {
() => {
// Module: crate::collections::btree::map
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the entries of a `BTreeMap`, sorted by key."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`BTreeMap`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: IntoIterator::into_iter"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_insignificant_dtor] pub struct IntoIter < K , V , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator + Clone = Global , > { range : LazyLeafRange < marker :: Dying , K , V > , length : usize , # [doc = " The BTreeMap will outlive this IntoIter so we don't care about drop order for `alloc`."] alloc : A , }
};
}
