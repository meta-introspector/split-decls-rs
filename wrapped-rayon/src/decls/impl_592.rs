macro_rules! deps {
    () => {
        IntoParallelIterator!();
        FromParallelIterator!();
    };
}

macro_rules! impl_592 {
    () => {
        deps!();
        # [doc = " Collects items from a parallel iterator into a vecdeque."] impl < T > FromParallelIterator < T > for VecDeque < T > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
    };
}

impl_592!()