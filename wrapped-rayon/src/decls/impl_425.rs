macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        # [doc = " Extends a hash map with items from a parallel iterator."] impl < K , V , S > ParallelExtend < (K , V) > for HashMap < K , V , S > where K : Eq + Hash + Send , V : Send , S : BuildHasher + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (K , V) > , { extend_reserved ! (self , par_iter) ; } }
    };
}

impl_425!()