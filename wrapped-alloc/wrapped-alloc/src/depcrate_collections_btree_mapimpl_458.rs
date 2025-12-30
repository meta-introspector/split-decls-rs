// Generated macro for impl_458 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_458 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_458"}
// Dependencies: {}
# [stable (feature = "default_iters_sequel" , since = "1.82.0")] impl < K , V > Default for RangeMut < '_ , K , V > { # [doc = " Creates an empty `btree_map::RangeMut`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::btree_map;"] # [doc = " let iter: btree_map::RangeMut<'_, u8, u8> = Default::default();"] # [doc = " assert_eq!(iter.count(), 0);"] # [doc = " ```"] fn default () -> Self { RangeMut { inner : Default :: default () , _marker : PhantomData } } }
};
}
