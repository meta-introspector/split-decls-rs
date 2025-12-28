macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        # [doc = " Extend a hash set with copied items from a parallel iterator."] impl < 'a , T , S > ParallelExtend < & 'a T > for HashSet < T , S , Global > where T : 'a + Copy + Eq + Hash + Sync , S : BuildHasher , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend (self , par_iter) ; } }
    };
}

impl_200!()