// Generated macro for impl_397 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_397 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_397"}
// Dependencies: {}
# [stable (feature = "default_iters" , since = "1.70.0")] impl < 'a , K : 'a , V : 'a > Default for IterMut < 'a , K , V > { # [doc = " Creates an empty `btree_map::IterMut`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::IterMut<'_, u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.len(), 0);"] # [doc = " ```"] fn default () -> Self { IterMut { range : Default :: default () , length : 0 , _marker : PhantomData { } } } }
};
}
