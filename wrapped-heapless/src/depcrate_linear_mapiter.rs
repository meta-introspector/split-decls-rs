// Generated macro for Iter (struct)
macro_rules! Depcrate_linear_mapIter {
() => {
// Module: crate::linear_map
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the items of a [`LinearMap`]"] # [doc = ""] # [doc = " This struct is created by calling the [`iter`](LinearMap::iter) method on [`LinearMap`]."] # [derive (Clone , Debug)] pub struct Iter < 'a , K , V > { iter : slice :: Iter < 'a , (K , V) > , }
};
}
