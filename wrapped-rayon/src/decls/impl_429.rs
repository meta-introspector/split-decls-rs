macro_rules! deps {
    () => {
        ListConsumer!();
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        # [doc = " Extends a linked list with items from a parallel iterator."] impl < T > ParallelExtend < T > for LinkedList < T > where T : Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { let mut list = par_iter . into_par_iter () . drive_unindexed (ListConsumer) ; self . append (& mut list) ; } }
    };
}

impl_429!()