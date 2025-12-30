// Generated macro for IterMut (struct)
macro_rules! Depcrate_linear_mapIterMut {
() => {
// Module: crate::linear_map
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " An iterator over the items of a [`LinearMap`] that allows modifying the items"] # [doc = ""] # [doc = " This struct is created by calling the [`iter_mut`](LinearMap::iter_mut) method on [`LinearMap`]."] # [derive (Debug)] pub struct IterMut < 'a , K , V > { iter : slice :: IterMut < 'a , (K , V) > , }
};
}
