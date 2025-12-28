macro_rules! deps {
    () => {
        IntoParallelIterator!();
        FromParallelIterator!();
    };
}

macro_rules! impl_598 {
    () => {
        deps!();
        # [doc = " Collects values from a parallel iterator into a btreeset."] impl < V > FromParallelIterator < V > for BTreeSet < V > where V : Send + Ord , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = V > , { collect_extended (par_iter) } }
    };
}

impl_598!();