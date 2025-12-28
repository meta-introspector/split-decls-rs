macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_591 {
    () => {
        deps!();
        # [doc = " Collects items from a parallel iterator into an atomically-reference-counted slice."] impl < T > FromParallelIterator < T > for Arc < [T] > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
    };
}

impl_591!()