macro_rules! deps {
    () => {
        IndexSet!();
        Bucket!();
    };
}

macro_rules! IntoParIter {
    () => {
        deps!();
        # [doc = " A parallel owning iterator over the items of an [`IndexSet`]."] # [doc = ""] # [doc = " This `struct` is created by the [`IndexSet::into_par_iter`] method"] # [doc = " (provided by rayon's [`IntoParallelIterator`] trait). See its documentation for more."] pub struct IntoParIter < T > { entries : Vec < Bucket < T > > , }
    };
}

IntoParIter!()