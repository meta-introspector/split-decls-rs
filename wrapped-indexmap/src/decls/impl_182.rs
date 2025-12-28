macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < 'a , T : 'a , S > ParallelExtend < & 'a T > for IndexSet < T , S > where T : Copy + Eq + Hash + Send + Sync , S : BuildHasher + Send , { fn par_extend < I > (& mut self , iter : I) where I : IntoParallelIterator < Item = & 'a T > , { for vec in collect (iter) { self . extend (vec) ; } } }
    };
}

impl_182!()