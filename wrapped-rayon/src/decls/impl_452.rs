macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        # [doc = " Extends a deque with items from a parallel iterator."] impl < T > ParallelExtend < T > for VecDeque < T > where T : Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend_reserved ! (self , par_iter) ; } }
    };
}

impl_452!()