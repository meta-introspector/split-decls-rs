macro_rules! deps {
    () => {
        IntoParallelIterator!();
        ParallelExtend!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        # [doc = " Extends a hash map with copied items from a parallel iterator."] impl < 'a , K : 'a , V : 'a , S > ParallelExtend < (& 'a K , & 'a V) > for HashMap < K , V , S > where K : Copy + Eq + Hash + Send + Sync , V : Copy + Send + Sync , S : BuildHasher + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (& 'a K , & 'a V) > , { extend_reserved ! (self , par_iter) ; } }
    };
}

impl_426!();