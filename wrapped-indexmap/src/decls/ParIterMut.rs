macro_rules! deps {
    () => {
        Bucket!();
        IndexMap!();
    };
}

macro_rules! ParIterMut {
    () => {
        deps!();
        # [doc = " A parallel mutable iterator over the entries of an [`IndexMap`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexMap::par_iter_mut`] method"] # [doc = " (provided by rayon's [`IntoParallelRefMutIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`IndexMap::par_iter_mut`]: ../struct.IndexMap.html#method.par_iter_mut"] pub struct ParIterMut < 'a , K , V > { entries : & 'a mut [Bucket < K , V >] , }
    };
}

ParIterMut!()