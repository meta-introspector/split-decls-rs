macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        # [doc = " Extends a hash set with copied items from a parallel iterator."] impl < 'a , T , S > ParallelExtend < & 'a T > for HashSet < T , S > where T : 'a + Copy + Eq + Hash + Send + Sync , S : BuildHasher + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend_reserved ! (self , par_iter) ; } }
    };
}

impl_428!()