macro_rules! deps {
    () => {
        IndexSet!();
        Bucket!();
    };
}

macro_rules! ParDrain {
    () => {
        deps!();
        # [doc = " A parallel draining iterator over the items of an [`IndexSet`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexSet::par_drain`] method"] # [doc = " (provided by rayon's [`ParallelDrainRange`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`IndexSet::par_drain`]: ../struct.IndexSet.html#method.par_drain"] pub struct ParDrain < 'a , T : Send > { entries : rayon :: vec :: Drain < 'a , Bucket < T > > , }
    };
}

ParDrain!();