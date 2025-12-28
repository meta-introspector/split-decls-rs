macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        # [doc = " Extends a B-tree set with items from a parallel iterator."] impl < T > ParallelExtend < T > for BTreeSet < T > where T : Ord + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend ! (self , par_iter) ; } }
    };
}

impl_423!();