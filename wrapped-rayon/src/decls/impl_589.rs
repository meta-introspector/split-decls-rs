macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_589 {
    () => {
        deps!();
        # [doc = " Collects items from a parallel iterator into a boxed slice."] impl < T > FromParallelIterator < T > for Box < [T] > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
    };
}

impl_589!();