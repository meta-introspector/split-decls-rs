// Generated macro for CursorMutKey (struct)
macro_rules! Depcrate_collections_btree_setCursorMutKey {
() => {
// Module: crate::collections::btree::set
// Provides: {"CursorMutKey"}
// Dependencies: {}
# [doc = " A cursor over a `BTreeSet` with editing operations, and which allows"] # [doc = " mutating elements."] # [doc = ""] # [doc = " A `Cursor` is like an iterator, except that it can freely seek back-and-forth, and can"] # [doc = " safely mutate the set during iteration. This is because the lifetime of its yielded"] # [doc = " references is tied to its own lifetime, instead of just the underlying set. This means"] # [doc = " cursors cannot yield multiple elements at once."] # [doc = ""] # [doc = " Cursors always point to a gap between two elements in the set, and can"] # [doc = " operate on the two immediately adjacent elements."] # [doc = ""] # [doc = " A `CursorMutKey` is created from a [`CursorMut`] with the"] # [doc = " [`CursorMut::with_mutable_key`] method."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Since this cursor allows mutating elements, you must ensure that the"] # [doc = " `BTreeSet` invariants are maintained. Specifically:"] # [doc = ""] # [doc = " * The newly inserted element must be unique in the tree."] # [doc = " * All elements in the tree must remain in sorted order."] # [unstable (feature = "btree_cursors" , issue = "107540")] pub struct CursorMutKey < 'a , K : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A = Global , > { inner : super :: map :: CursorMutKey < 'a , K , SetValZST , A > , }
};
}
