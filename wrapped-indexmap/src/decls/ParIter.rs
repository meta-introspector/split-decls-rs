macro_rules! deps {
    () => {
        IndexSet!();
        Bucket!();
    };
}

macro_rules! ParIter {
    () => {
        deps!();
        # [doc = " A parallel iterator over the items of an [`IndexSet`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexSet::par_iter`] method"] # [doc = " (provided by rayon's [`IntoParallelRefIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`IndexSet::par_iter`]: ../struct.IndexSet.html#method.par_iter"] pub struct ParIter < 'a , T > { entries : & 'a [Bucket < T >] , }
    };
}

ParIter!();