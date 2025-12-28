macro_rules! collect {
    () => {
        fn collect < I : IntoParallelIterator > (iter : I) -> LinkedList < Vec < I :: Item > > { iter . into_par_iter () . collect_vec_list () }
    };
}

collect!();