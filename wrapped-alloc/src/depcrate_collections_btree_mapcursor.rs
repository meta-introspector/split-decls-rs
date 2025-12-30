// Generated macro for Cursor (struct)
macro_rules! Depcrate_collections_btree_mapCursor {
() => {
// Module: crate::collections::btree::map
// Provides: {"Cursor"}
// Dependencies: {}
# [doc = " A cursor over a `BTreeMap`."] # [doc = ""] # [doc = " A `Cursor` is like an iterator, except that it can freely seek back-and-forth."] # [doc = ""] # [doc = " Cursors always point to a gap between two elements in the map, and can"] # [doc = " operate on the two immediately adjacent elements."] # [doc = ""] # [doc = " A `Cursor` is created with the [`BTreeMap::lower_bound`] and [`BTreeMap::upper_bound`] methods."] # [unstable (feature = "btree_cursors" , issue = "107540")] pub struct Cursor < 'a , K : 'a , V : 'a > { current : Option < Handle < NodeRef < marker :: Immut < 'a > , K , V , marker :: Leaf > , marker :: Edge > > , root : Option < & 'a node :: Root < K , V > > , }
};
}
