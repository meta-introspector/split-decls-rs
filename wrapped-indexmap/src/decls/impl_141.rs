macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'a , K : 'a , V : 'a , S > ParallelExtend < (& 'a K , & 'a V) > for IndexMap < K , V , S > where K : Copy + Eq + Hash + Send + Sync , V : Copy + Send + Sync , S : BuildHasher + Send , { fn par_extend < I > (& mut self , iter : I) where I : IntoParallelIterator < Item = (& 'a K , & 'a V) > , { for vec in collect (iter) { self . extend (vec) ; } } }
    };
}

impl_141!();