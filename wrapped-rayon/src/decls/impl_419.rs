macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        # [doc = " Extends a binary heap with items from a parallel iterator."] impl < T > ParallelExtend < T > for BinaryHeap < T > where T : Ord + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend_reserved ! (self , par_iter) ; } }
    };
}

impl_419!();