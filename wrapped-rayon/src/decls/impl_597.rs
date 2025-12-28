macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_597 {
    () => {
        deps!();
        # [doc = " Collects values from a parallel iterator into a hashset."] impl < V , S > FromParallelIterator < V > for HashSet < V , S > where V : Eq + Hash + Send , S : BuildHasher + Default + Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = V > , { collect_extended (par_iter) } }
    };
}

impl_597!();