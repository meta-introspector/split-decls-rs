macro_rules! deps {
    () => {
        IntoParallelIterator!();
        FromParallelIterator!();
    };
}

macro_rules! impl_590 {
    () => {
        deps!();
        # [doc = " Collects items from a parallel iterator into a reference-counted slice."] impl < T > FromParallelIterator < T > for Rc < [T] > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
    };
}

impl_590!()