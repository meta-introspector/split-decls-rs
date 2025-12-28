macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_453 {
    () => {
        deps!();
        # [doc = " Extends a deque with copied items from a parallel iterator."] impl < 'a , T > ParallelExtend < & 'a T > for VecDeque < T > where T : 'a + Copy + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend_reserved ! (self , par_iter) ; } }
    };
}

impl_453!();