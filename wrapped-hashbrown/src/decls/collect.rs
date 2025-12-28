macro_rules! collect {
    () => {
        # [doc = " Helper for collecting parallel iterators to an intermediary"] # [allow (clippy :: linkedlist)] pub (super) fn collect < I : IntoParallelIterator > (iter : I) -> (LinkedList < Vec < I :: Item > > , usize) { let list = iter . into_par_iter () . collect_vec_list () ; let len = list . iter () . map (Vec :: len) . sum () ; (list , len) }
    };
}

collect!()