macro_rules! deps {
    () => {
        ParallelExtend!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        # [doc = " Extends a B-tree map with items from a parallel iterator."] impl < K , V > ParallelExtend < (K , V) > for BTreeMap < K , V > where K : Ord + Send , V : Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (K , V) > , { extend ! (self , par_iter) ; } }
    };
}

impl_421!()