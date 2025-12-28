macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < T , S > ParallelExtend < T > for IndexSet < T , S > where T : Eq + Hash + Send , S : BuildHasher + Send , { fn par_extend < I > (& mut self , iter : I) where I : IntoParallelIterator < Item = T > , { for vec in collect (iter) { self . extend (vec) ; } } }
    };
}

impl_181!()