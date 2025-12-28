macro_rules! deps {
    () => {
        IntoParallelIterator!();
        FromParallelIterator!();
    };
}

macro_rules! impl_596 {
    () => {
        deps!();
        # [doc = " Collects (key, value) pairs from a parallel iterator into a"] # [doc = " btreemap. If multiple pairs correspond to the same key, then the"] # [doc = " ones produced earlier in the parallel iterator will be"] # [doc = " overwritten, just as with a sequential iterator."] impl < K , V > FromParallelIterator < (K , V) > for BTreeMap < K , V > where K : Ord + Send , V : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = (K , V) > , { collect_extended (par_iter) } }
    };
}

impl_596!();