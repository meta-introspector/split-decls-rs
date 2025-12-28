macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        # [doc = " Extends a binary heap with copied items from a parallel iterator."] impl < 'a , T > ParallelExtend < & 'a T > for BinaryHeap < T > where T : 'a + Copy + Ord + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend_reserved ! (self , par_iter) ; } }
    };
}

impl_420!()