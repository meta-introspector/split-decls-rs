macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        # [doc = " Extend a hash map with items from a parallel iterator."] impl < K , V , S , A > ParallelExtend < (K , V) > for HashMap < K , V , S , A > where K : Eq + Hash + Send , V : Send , S : BuildHasher , A : Allocator , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (K , V) > , { extend (self , par_iter) ; } }
    };
}

impl_155!()