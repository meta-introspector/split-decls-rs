macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! collect_into_vec {
    () => {
        deps!();
        # [doc = " Collects the results of the exact iterator into the specified vector."] # [doc = ""] # [doc = " This is called by `IndexedParallelIterator::collect_into_vec`."] pub (super) fn collect_into_vec < I , T > (pi : I , v : & mut Vec < T >) where I : IndexedParallelIterator < Item = T > , T : Send , { v . truncate (0) ; let len = pi . len () ; collect_with_consumer (v , len , | consumer | pi . drive (consumer)) ; }
    };
}

collect_into_vec!()