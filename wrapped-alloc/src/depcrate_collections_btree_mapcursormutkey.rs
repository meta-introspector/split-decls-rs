// Generated macro for CursorMutKey (struct)
macro_rules! Depcrate_collections_btree_mapCursorMutKey {
() => {
// Module: crate::collections::btree::map
// Provides: {"CursorMutKey"}
// Dependencies: {}
# [doc = " A cursor over a `BTreeMap` with editing operations, and which allows"] # [doc = " mutating the key of elements."] # [doc = ""] # [doc = " A `Cursor` is like an iterator, except that it can freely seek back-and-forth, and can"] # [doc = " safely mutate the map during iteration. This is because the lifetime of its yielded"] # [doc = " references is tied to its own lifetime, instead of just the underlying map. This means"] # [doc = " cursors cannot yield multiple elements at once."] # [doc = ""] # [doc = " Cursors always point to a gap between two elements in the map, and can"] # [doc = " operate on the two immediately adjacent elements."] # [doc = ""] # [doc = " A `CursorMutKey` is created from a [`CursorMut`] with the"] # [doc = " [`CursorMut::with_mutable_key`] method."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Since this cursor allows mutating keys, you must ensure that the `BTreeMap`"] # [doc = " invariants are maintained. Specifically:"] # [doc = ""] # [doc = " * The key of the newly inserted element must be unique in the tree."] # [doc = " * All keys in the tree must remain in sorted order."] # [unstable (feature = "btree_cursors" , issue = "107540")] pub struct CursorMutKey < 'a , K : 'a , V : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A = Global , > { current : Option < Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge > > , root : DormantMutRef < 'a , Option < node :: Root < K , V > > > , length : & 'a mut usize , alloc : & 'a mut A , }
};
}
