macro_rules! deps {
    () => {
        IntoParallelIterator!();
        FromParallelIterator!();
    };
}

macro_rules! impl_593 {
    () => {
        deps!();
        # [doc = " Collects items from a parallel iterator into a binaryheap."] # [doc = " The heap-ordering is calculated serially after all items are collected."] impl < T > FromParallelIterator < T > for BinaryHeap < T > where T : Ord + Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
    };
}

impl_593!()