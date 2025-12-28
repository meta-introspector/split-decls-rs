macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        # [doc = " Extend a hash map with copied items from a parallel iterator."] impl < 'a , K , V , S , A > ParallelExtend < (& 'a K , & 'a V) > for HashMap < K , V , S , A > where K : Copy + Eq + Hash + Sync , V : Copy + Sync , S : BuildHasher , A : Allocator , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (& 'a K , & 'a V) > , { extend (self , par_iter) ; } }
    };
}

impl_156!()