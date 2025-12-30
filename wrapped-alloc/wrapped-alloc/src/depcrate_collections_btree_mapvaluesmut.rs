// Generated macro for ValuesMut (struct)
macro_rules! Depcrate_collections_btree_mapValuesMut {
() => {
// Module: crate::collections::btree::map
// Provides: {"ValuesMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the values of a `BTreeMap`."] # [doc = ""] # [doc = " This `struct` is created by the [`values_mut`] method on [`BTreeMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`values_mut`]: BTreeMap::values_mut"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "map_values_mut" , since = "1.10.0")] pub struct ValuesMut < 'a , K , V > { inner : IterMut < 'a , K , V > , }
};
}
