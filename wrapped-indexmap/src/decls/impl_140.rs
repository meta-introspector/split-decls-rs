macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < K , V , S > ParallelExtend < (K , V) > for IndexMap < K , V , S > where K : Eq + Hash + Send , V : Send , S : BuildHasher + Send , { fn par_extend < I > (& mut self , iter : I) where I : IntoParallelIterator < Item = (K , V) > , { for vec in collect (iter) { self . extend (vec) ; } } }
    };
}

impl_140!()