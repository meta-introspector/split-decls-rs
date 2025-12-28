macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        # [doc = " Extend a hash set with items from a parallel iterator."] impl < T , S > ParallelExtend < T > for HashSet < T , S , Global > where T : Eq + Hash + Send , S : BuildHasher , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend (self , par_iter) ; } }
    };
}

impl_199!()