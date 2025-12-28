macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        # [doc = " Extends a B-tree set with copied items from a parallel iterator."] impl < 'a , T > ParallelExtend < & 'a T > for BTreeSet < T > where T : 'a + Copy + Ord + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend ! (self , par_iter) ; } }
    };
}

impl_424!();