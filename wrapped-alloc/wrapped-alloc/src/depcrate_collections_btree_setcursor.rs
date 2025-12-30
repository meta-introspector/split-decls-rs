// Generated macro for Cursor (struct)
macro_rules! Depcrate_collections_btree_setCursor {
() => {
// Module: crate::collections::btree::set
// Provides: {"Cursor"}
// Dependencies: {}
# [doc = " A cursor over a `BTreeSet`."] # [doc = ""] # [doc = " A `Cursor` is like an iterator, except that it can freely seek back-and-forth."] # [doc = ""] # [doc = " Cursors always point to a gap between two elements in the set, and can"] # [doc = " operate on the two immediately adjacent elements."] # [doc = ""] # [doc = " A `Cursor` is created with the [`BTreeSet::lower_bound`] and [`BTreeSet::upper_bound`] methods."] # [derive (Clone)] # [unstable (feature = "btree_cursors" , issue = "107540")] pub struct Cursor < 'a , K : 'a > { inner : super :: map :: Cursor < 'a , K , SetValZST > , }
};
}
