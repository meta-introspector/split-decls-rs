macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_594 {
    () => {
        deps!();
        # [doc = " Collects items from a parallel iterator into a freshly allocated"] # [doc = " linked list."] impl < T > FromParallelIterator < T > for LinkedList < T > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { collect_extended (par_iter) } }
    };
}

impl_594!()