// Generated macro for ParDrain (struct)
macro_rules! Depcrate_rayon_mapParDrain {
() => {
// Module: crate::rayon::map
// Provides: {"ParDrain"}
// Dependencies: {}
# [doc = " A parallel draining iterator over the entries of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_drain`] method"] # [doc = " (provided by rayon's [`ParallelDrainRange`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`IndexMap::par_drain`]: ../struct.IndexMap.html#method.par_drain"] pub struct ParDrain < 'a , K : Send , V : Send > { entries : rayon :: vec :: Drain < 'a , Bucket < K , V > > , }
};
}
