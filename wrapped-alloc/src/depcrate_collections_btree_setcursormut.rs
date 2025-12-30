// Generated macro for CursorMut (struct)
macro_rules! Depcrate_collections_btree_setCursorMut {
() => {
// Module: crate::collections::btree::set
// Provides: {"CursorMut"}
// Dependencies: {}
# [doc = " A cursor over a `BTreeSet` with editing operations."] # [doc = ""] # [doc = " A `Cursor` is like an iterator, except that it can freely seek back-and-forth, and can"] # [doc = " safely mutate the set during iteration. This is because the lifetime of its yielded"] # [doc = " references is tied to its own lifetime, instead of just the underlying map. This means"] # [doc = " cursors cannot yield multiple elements at once."] # [doc = ""] # [doc = " Cursors always point to a gap between two elements in the set, and can"] # [doc = " operate on the two immediately adjacent elements."] # [doc = ""] # [doc = " A `CursorMut` is created with the [`BTreeSet::lower_bound_mut`] and [`BTreeSet::upper_bound_mut`]"] # [doc = " methods."] # [unstable (feature = "btree_cursors" , issue = "107540")] pub struct CursorMut < 'a , K : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A = Global > { inner : super :: map :: CursorMut < 'a , K , SetValZST , A > , }
};
}
