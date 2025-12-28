macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! collect_extended {
    () => {
        deps!();
        # [doc = " Creates an empty default collection and extends it."] fn collect_extended < C , I > (par_iter : I) -> C where I : IntoParallelIterator , C : ParallelExtend < I :: Item > + Default , { let mut collection = C :: default () ; collection . par_extend (par_iter) ; collection }
    };
}

collect_extended!()