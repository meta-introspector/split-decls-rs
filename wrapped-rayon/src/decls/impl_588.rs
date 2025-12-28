macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        # [doc = " Collects items from a parallel iterator into a vector."] impl < T > FromParallelIterator < T > for Vec < T > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { collect_extended (par_iter) } }
    };
}

impl_588!();