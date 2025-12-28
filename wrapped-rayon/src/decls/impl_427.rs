macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        # [doc = " Extends a hash set with items from a parallel iterator."] impl < T , S > ParallelExtend < T > for HashSet < T , S > where T : Eq + Hash + Send , S : BuildHasher + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend_reserved ! (self , par_iter) ; } }
    };
}

impl_427!();